use std::rc::Rc;
use cgmath::{Quaternion, Matrix4, Vector2, vec3, vec4, SquareMatrix, vec2};
use downcast_rs::{Downcast, impl_downcast};
use silver_gl::{Texture, ShaderProgram, Mesh};
use crate::{EngineError, ResourceManager};

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

pub trait Widget: Downcast {
    // Getters and Setters (required properties essentially)
    fn get_position(&self) -> Vector2<f32>;
    fn set_position(&mut self, pos: Vector2<f32>);
    fn get_rotation(&self) -> Quaternion<f32>;
    fn set_rotation(&mut self, rot: Quaternion<f32>);
    // TODO: Add size struct, with all the pixel things
    fn get_size(&self) -> (f32, f32);
    fn set_size(&mut self, width: f32, height: f32);
    fn get_children(&self) -> &Vec<Box<dyn Widget>>;
    fn get_children_mut(&mut self) -> &mut Vec<Box<dyn Widget>>;
    // Used for relative widgets
    fn get_vec_space(&self) -> Matrix4<f32>;
    fn set_vec_space(&mut self, vec_space: Matrix4<f32>);
    // All in one setter to simplify process of setting textures
    // This should be the only way to set textures, since batching isn't something you need
    // to worry about.
    // Only used for texure primitive widgets
    fn get_texture(&self) -> &Option<Rc<Texture>> { &None } // Used for texture primitive widgets
    fn set_texture(&mut self, texture: Rc<Texture>) -> Result<(), EngineError> { Err(EngineError::TexturelessWidget(texture.get_id())) }
    fn get_shader_program(&self) -> &Rc<ShaderProgram>;
    fn set_shader_program(&mut self, shader_program: Rc<ShaderProgram>);

    // Calculates transform matrix for the vertex shader
    fn transform_matrix(&self) -> Matrix4<f32> {
        let pos = self.get_position();
        let mut matrix = Matrix4::<f32>::from_translation(vec3(pos.x, pos.y, 0.0));
        let (width, height) = self.get_size();
        matrix = matrix * Matrix4::<f32>::from_nonuniform_scale(width, height, 1.0);
        matrix = matrix * Matrix4::<f32>::from(self.get_rotation());
        // TODO: Add option to change where the widget is rotated from
        // TODO: This could be implemented by transforming by relative points then rotating
        matrix = self.get_vec_space() * matrix;
        
        matrix
    }

    // Transforms between screen-space pixels and in-engine positions
    // TODO: when doing the point approach, rename these to point functions, then have a top-top layer widget
    // TODO: that converts from points to pixels with accompanying conversion functions.
    // TODO: Point could rather be just a simple constant which multiplies from position (points) to position
    // TODO: (resolution). These are just then in a position struct/enum that can convert between and is
    // TODO: passed between things.
    fn get_size_pixels(&self) -> (f32, f32) {
        let (width, height) = self.get_size();
        let mut size_vec = vec4(width, height, 0.0, 1.0);

        size_vec = self.get_vec_space() * size_vec;

        (size_vec.x, size_vec.y)
    }
    fn set_size_pixels(&mut self, width: f32, height: f32) {
        let mut size_vec = vec4(width, height, 0.0, 1.0);

        let inverted_vec_space = self.get_vec_space().invert().expect("Transformation matrix should be invertible");
        size_vec = inverted_vec_space * size_vec;

        self.set_size(size_vec.x, size_vec.y);
    }
    fn get_position_pixels(&self) -> Vector2<f32> {
        let pos = self.get_position();
        let mut pos_vec = vec4(pos.x, pos.y, 0.0, 1.0);

        pos_vec = self.get_vec_space() * pos_vec;

        vec2(pos_vec.x, pos_vec.y)
    }
    fn set_position_pixels(&mut self, pos: Vector2<f32>) {
        let mut pos_vec = vec4(pos.x, pos.y, 0.0, 1.0);

        let inverted_vec_space = self.get_vec_space().invert().expect("Transformation matrix should be invertible");
        pos_vec = inverted_vec_space * pos_vec;

        self.set_position(vec2(pos_vec.x, pos_vec.y));
    }

    // Send visual properties of the widget to shader program
    fn update_shader_program(&mut self);

    fn draw(&self) -> Result<(), EngineError> {
        // Draw bottom-most widgets first
        for widget in self.get_children() {
            
        }

        Ok(())
    }
}

// Instead of using an enum to remove downcasting, this allows for a new widget
// to be created by anyone who uses the engine.
// A third party crate is used here for more flexibility with downcasting
impl_downcast!(Widget);