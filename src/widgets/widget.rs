use std::rc::Rc;
use cgmath::{Quaternion, Matrix4, Vector2, vec3, vec4, SquareMatrix, vec2};
use downcast_rs::{Downcast, impl_downcast};
use silver_gl::{Texture, ShaderProgram, MultiBindModel, ModelTrait};
use crate::EngineError;

// TODO: To create a kind of crosshair widget that follows a point (to make animating in widgets cool),
// TODO: have a widget (child of top-level widget, preferably on the bottom of the vec so it draws
// TODO: over everything else) that is a new type of primitive widget. Its main (most likely only)
// TODO: properties will be widths and colours of all 4 lines (+/- x/y), a length, where they are
// TODO: centered (with offset), all allowing pixel sizes as well. All these options should allow
// TODO: fairly complex animations for drawing in widgets.
// TODO: Possibly have a composite helper widget that creates and destroys crosshair widgets to allow
// TODO: for dead simple support of animating in multiple widgets. Takes a definition of the animation,
// TODO: when initialized, then to animate the crosshair takes position and done value in [0,1].

// TODO: Create a system where you have a target aspect ratio (for example 16:9), and then the screen
// TODO: will be divided up into "dots", mimicing screen-resolution in its function. Resolution can
// TODO: can be set when creating the engine. An example would be 16:9 aspect ratio with 100 resolution,
// TODO: resulting in dots of 1600x900, which would remain constant no matter the screen resolution.

// TODO: Make an enum for the 3 types of positioning coordinates. These hold the values in their
// TODO: respective forms, each floats. Each option will be a point with (x,y). Widgets store only
// TODO: relative locations. Transform the get/set_position/size functions into _inner, so that
// TODO: the implementer only needs to implement relative sizing. Then make more general default
// TODO: functions where you can set the enum (and it will automatically translate), or with
// TODO: get_*_pixel/dot/relative. The dot system will be initialized in the engine, and will
// TODO: give a transform matrix to go from pixels to dots. Possibly use a type alias to change
// TODO: Point enum to Size, but they both essentially are the same thing. Possibly implement
// TODO: From traits on each of them to translate between the two?

// TODO: Find ways to optimize traverse_and_push_all by only pushing/dealing with changes

// TODO: Find a way to do widget shaddows (draw a second quad w/ Gaussian blur underneath?)
// TODO: Has both size and offset, so you can customize the shadow to your liking!

// TODO: Line widget, you set its start and end points, or len/start/angle
// TODO: Use line widgets to create a magify widget, where you have a
// TODO: box around something, a bigger version of it elsewhere, and lines connecting the
// TODO: corners

// TODO: Defaults aren't handled by the engine, rather they are handled by the script then
// TODO: used in the interpreter. The script directly links to shader programs for example

// TODO: All widget pre-done shaders assume square

pub trait Widget: Downcast {
    // Getters and Setters (required properties essentially)
    // Pos/size are in [-1,1] relative to parent widget, so resizing to fit window change is
    // unneccesary
    fn get_position(&self) -> Vector2<f32>;
    fn set_position(&mut self, pos: Vector2<f32>);

    fn get_rotation(&self) -> Quaternion<f32>;
    fn set_rotation(&mut self, rot: Quaternion<f32>);

    // TODO: Add size struct, with all the pixel things
    fn get_size(&self) -> (f32, f32);
    fn set_size(&mut self, width: f32, height: f32);

    fn get_children(&self) -> &Vec<Box<dyn Widget>>;
    fn get_children_mut(&mut self) -> &mut Vec<Box<dyn Widget>>;

    // All in one setter to simplify process of setting textures
    // This should be the only way to set textures, since batching isn't something you need
    // to worry about.
    // Only used for texure primitive widgets
    fn get_texture(&self) -> Option<&Rc<Texture>> { None }
    fn set_texture(&mut self, texture: Rc<Texture>) -> Result<(), EngineError> { Err(EngineError::TexturelessWidget(texture.get_id())) }
    
    fn get_shader_program(&self) -> &Rc<ShaderProgram>;
    fn set_shader_program(&mut self, shader_program: Rc<ShaderProgram>);

    fn get_model(&self) -> &MultiBindModel;
    fn get_model_mut(&mut self) -> &mut MultiBindModel;
    fn set_model(&mut self, model: MultiBindModel);

    fn get_vec_space(&self) -> Matrix4<f32>;
    fn set_vec_space(&mut self, vec_space: Matrix4<f32>);

    // Calculates transform matrix for the vertex shader
    fn transform_matrix(&self) -> Matrix4<f32> {
        let pos = self.get_position();
        let mut matrix = Matrix4::<f32>::from_translation(vec3(pos.x, pos.y, 0.0));
        let (width, height) = self.get_size();
        matrix = matrix * Matrix4::<f32>::from_nonuniform_scale(width, height, 1.0);
        matrix = matrix * Matrix4::<f32>::from(self.get_rotation());
        // TODO: Add option to change where the widget is rotated from
        // TODO: This could be implemented by transforming by relative points then rotating
        
        matrix
    }

    // Transforms between screen-space pixels/dots and in-engine positions in [-1,1]
    fn get_size_from_res(&self, screen_width: i32, screen_height: i32) -> (f32, f32) {
        let (width, height) = self.get_size();
        let mut size_vec = vec4(width, height, 0.0, 1.0);

        // Convert to range of [-1,1] in screen space
        size_vec = self.get_vec_space() * size_vec;
        // Convert from [-1,1] to provided resolution scale
        size_vec.x *= screen_width as f32;
        size_vec.y *= screen_height as f32;

        (size_vec.x, size_vec.y)
    }
    fn set_size_from_res(&mut self, width: f32, height: f32, screen_width: i32, screen_height: i32) {
        let mut size_vec = vec4(width, height, 0.0, 1.0);
        // Convert from provided resolution scale to [-1,1]
        size_vec.x /= screen_width as f32;
        size_vec.y /= screen_height as f32;

        // Convert to widget's vec space
        let inverted_vec_space = self.get_vec_space()
            .invert()
            .expect("Transformation matrix should be invertible");
        size_vec = inverted_vec_space * size_vec;

        self.set_size(size_vec.x, size_vec.y);
    }
    fn get_position_from_res(&self, screen_width: i32, screen_height: i32) -> Vector2<f32> {
        let pos = self.get_position();
        let mut pos_vec = vec4(pos.x, pos.y, 0.0, 1.0);

        // Convert to range of [-1,1] in screen space
        pos_vec = self.get_vec_space() * pos_vec;
        // Convert from [-1,1] to provided resolution scale
        pos_vec.x = ((pos_vec.x + 1.0) / 2.0) * screen_width as f32;
        pos_vec.y = ((pos_vec.y + 1.0) / 2.0) * screen_height as f32;

        vec2(pos_vec.x, pos_vec.y)
    }
    fn set_position_from_res(&mut self, pos: Vector2<f32>, screen_width: i32, screen_height: i32) {
        let mut pos_vec = vec4(pos.x, pos.y, 0.0, 1.0);
        // Convert from provided resolution scale to [-1,1]
        pos_vec.x = ((pos_vec.x / screen_width as f32) * 2.0) - 1.0;
        pos_vec.y = ((pos_vec.y / screen_height as f32) * 2.0) - 1.0;

        // Convert to widget's vec space
        let inverted_vec_space = self.get_vec_space()
            .invert()
            .expect("Transformation matrix should be invertible");
        pos_vec = inverted_vec_space * pos_vec;

        self.set_position(vec2(pos_vec.x, pos_vec.y));
    }

    // Send visual properties of the widget to shader program
    // Shader program is bound beforehand so don't rebind
    // Only unique props need to be set here
    fn update_shader_program(&self) -> Result<(), EngineError>;

    // Draw command should be called from top down, in which case always use
    // Matrix4::identity()
    fn draw(&mut self, vec_space: Matrix4<f32>) -> Result<(), EngineError> {
        self.set_vec_space(vec_space);
        let transform_matrix = vec_space * self.transform_matrix();

        // Draw bottom-most widgets first
        for widget in self.get_children_mut() {
            widget.draw(transform_matrix)?;
        }

        let sp = self.get_shader_program();

        sp.use_program();

        // Ignores whether the properties are present so that the pre-made widgets
        // "make available" certain props
        unsafe {
            sp.set_mat4_unsafe("transform", &transform_matrix)?;
        }

        self.update_shader_program()?;
        self.get_model().draw(sp)?;

        Ok(())
    }
}

// Instead of using an enum to remove downcasting, this allows for a new widget
// to be created by anyone who uses the engine.
// A third party crate is used here for more flexibility with downcasting
impl_downcast!(Widget);