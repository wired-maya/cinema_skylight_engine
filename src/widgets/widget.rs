use std::rc::Rc;
use cgmath::{Quaternion, Matrix4, Vector2, vec3, vec4, SquareMatrix, vec2};
use downcast_rs::{Downcast, impl_downcast};
use silver_gl::{Model, Texture, ShaderProgram};
use crate::{EngineError, primitives::PrimitiveCounter};

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

// TODO: NEW METHOD OF RENDERING WIDGETS!
// TODO: Old way was confusing and not as extensible as it should be. New way should be significantly
// TODO: better. Basic Idea:
// TODO: Each primitive widget gets 1KB of data in the GPU memory, after transformation matrices. This
// TODO: contains a widget type at the start (u8 with primitives being assigned different codes), a
// TODO: length in bytes (u32), and the rest of it will be data. The fragment shader can then read the
// TODO: primitive code, then pass the blob to the appropriate render function where it is coerced
// TODO: into the appropriate type (struct that has things like border width and colour, for example),
// TODO: then rendered appropriately. This will allow significantly greater texture binds, less
// TODO: complexity with not needing a primitive counter, and the same flexibility to create new
// TODO: primitive widget (just start with a primitive code of the engine reserved codes + 1)
pub trait Widget: Downcast {
    fn get_position(&self) -> Vector2<f32>;
    fn set_position(&mut self, pos: Vector2<f32>);
    fn get_rotation(&self) -> Quaternion<f32>;
    fn set_rotation(&mut self, rot: Quaternion<f32>);
    // TODO: Add size struct, with all the pixel things
    fn get_size(&self) -> (f32, f32);
    fn set_size(&mut self, width: f32, height: f32);
    fn get_children(&self) -> &Vec<Box<dyn Widget>>;
    fn get_children_mut(&mut self) -> &mut Vec<Box<dyn Widget>>;

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
    
    // Transforms current size to pixels
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

    // These are used to optimize changing textures and transforms
    fn get_index(&self) -> Option<usize>;
    fn set_index(&mut self, i: Option<usize>);

    // Used for relative widgets
    fn get_vec_space(&self) -> Matrix4<f32>;
    fn set_vec_space(&mut self, vec_space: Matrix4<f32>);

    // Send visual properties of the widget to shader program
    fn send_widget_info(&self, shader_program: &ShaderProgram, counter: &mut PrimitiveCounter) -> Result<(), EngineError>;

    // Traverses tree in linearized order, pushing the widgets' information to the quad
    // Assumes that clear_inner has been run on the tbo and clear has been run on the
    // mesh's diffuse textures, that send_data_mut is ran afterwards, and that a shader
    // program is bound.
    // Panics if there are less than 1 meshes in quad.
    // Needs to be run when widget tree is changed
    fn traverse_and_push_all(
        &mut self,
        quad: &mut Model,
        shader_program: &ShaderProgram,
        vec_space: Matrix4<f32>,
        counter: &mut PrimitiveCounter
    ) -> Result<(), EngineError> {
        self.set_index(Some(quad.tbo.len()));
        self.set_vec_space(vec_space);
        
        let matrix = self.transform_matrix();

        unsafe {
            quad.tbo.push_to_inner(matrix);
        }

        if let Some(texture) = self.get_texture() {
            quad.meshes[0].diffuse_textures.push(Rc::clone(texture));
        }

        self.send_widget_info(shader_program, counter)?;

        for widget in self.get_children_mut() {
            widget.traverse_and_push_all(quad, shader_program, matrix, counter)?;
        }

        Ok(())
    }

    // Index needs to be set, meaning above function needs to have been run
    // Cannot be run after widget tree is modified without push_all being run first
    // To Used to batch together transforms
    // This is meant to to be run with a send_data afterwards, since it's to batch
    // transforms
    fn traverse_and_set_transforms(&mut self, quad: &mut Model, vec_space: Matrix4<f32>) -> Result<(), EngineError> {
        self.set_vec_space(vec_space);

        let matrix = self.transform_matrix();

        if let Some(index) = self.get_index() {
            unsafe {
                quad.tbo.set_data_index_inner(matrix, index);
            }
        } else {
            return Err(EngineError::WidgetIndexMissing());
        }

        for widget in self.get_children_mut() {
            widget.traverse_and_set_transforms(quad, matrix)?;
        }
        
        Ok(())
    }

    fn traverse_and_set_textures(&self, quad: &mut Model) -> Result<(), EngineError> {
        if let (Some(index), Some(texture)) = (self.get_index(), self.get_texture()) {
            quad.meshes[0].diffuse_textures[index] = Rc::clone(texture);
        } else {
            return Err(EngineError::WidgetIndexMissing());
        }

        for widget in self.get_children() {
            widget.traverse_and_set_textures(quad)?;
        }
        
        Ok(())
    }

    // These set inner transforms and textures, so you can batch-transform a selection
    // of widgets without sacrificing performance
    // Requires a send_data to be send afterwards
    fn set_transform(&self, quad: &mut Model) -> Result<(), EngineError> {
        if let Some(index) = self.get_index() {
            unsafe {
                quad.tbo.set_data_index_inner(self.transform_matrix(), index);
            }
        } else {
            return Err(EngineError::WidgetIndexMissing());
        }
        
        Ok(())
    }

    // Sets index properties
    // This are meant to be run standalone, so only when updating one thing
    // If performance is an issue, use one of the above
    // Doesn't have all in one transform because of the 3 things that make it up
    fn set_transform_send(&self, quad: &mut Model) -> Result<(), EngineError> {
        if let Some(index) = self.get_index() {
            quad.tbo.set_data_index(self.transform_matrix(), index);
        } else {
            return Err(EngineError::WidgetIndexMissing());
        }
        
        Ok(())
    }

    // Textures are always sent to the shader each frame, so this doesn't need any subsequent
    // function
    fn set_texture_send(&self, quad: &mut Model) -> Result<(), EngineError> {
        if let (Some(index), Some(texture)) = (self.get_index(), self.get_texture()) {
            quad.meshes[0].diffuse_textures[index] = Rc::clone(texture);
        } else {
            return Err(EngineError::WidgetIndexMissing());
        }
        
        Ok(())
    }

    // All in one setter to simplify process of setting textures
    // This should be the only way to set textures, since batching isn't something you need
    // to worry about.
    // Only used for texure primitive widgets
    fn get_texture(&self) -> &Option<Rc<Texture>> { &None } // Used for texture primitive widgets
    fn set_texture(&mut self, texture: Rc<Texture>) -> Result<(), EngineError> { Err(EngineError::TexturelessWidget(texture.get_id())) }

    // Needs to be run each frame
    fn traverse_and_send_info(&self, shader_program: &ShaderProgram, counter: &mut PrimitiveCounter) -> Result<(), EngineError> {
        self.send_widget_info(shader_program, counter)?;

        for widget in self.get_children() {
            widget.traverse_and_send_info(shader_program, counter)?;
        }

        Ok(())
    }
}

// Instead of using an enum to remove downcasting, this allows for a new widget
// to be created by anyone who uses the engine.
// A third party crate is used here for more flexibility with downcasting
impl_downcast!(Widget);