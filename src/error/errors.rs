use std::{fmt::Display, error::Error};

use silver_gl::GlError;

#[derive(Debug)]
pub enum EngineError {
    ObjLoadError(tobj::LoadError),
    ImageError(image::ImageError),
    IoError(std::io::Error),
    GlError(GlError),
    WidgetIndexMissing(),
    TexturelessWidget(u32)
}

// TODO: Write errors that suggest a solution as well
impl Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineError::ObjLoadError(obj_err) => write!(f, "{}", obj_err),
            EngineError::ImageError(img_err) => write!(f, "{}", img_err),
            EngineError::IoError(io_err) => write!(f, "{}", io_err),
            EngineError::GlError(gl_err) => write!(f, "{}", gl_err),
            EngineError::WidgetIndexMissing() => write!(f, "The index for this widget does not exist.\nThis occurs when you haven't run traverse_and_push() after modifying the widget tree"),
            EngineError::TexturelessWidget(id) => write!(f, "This widget does not take a texuture, yet the texture {id} was provided", )
        }
    }
}

impl Error for EngineError {}

impl From<tobj::LoadError> for EngineError {
    fn from(err: tobj::LoadError) -> Self {
        EngineError::ObjLoadError(err)
    }
}

impl From<image::ImageError> for EngineError {
    fn from(err: image::ImageError) -> Self {
        EngineError::ImageError(err)
    }
}

impl From<std::io::Error> for EngineError {
    fn from(err: std::io::Error) -> Self {
        EngineError::IoError(err)
    }
}

impl From<GlError> for EngineError {
    fn from(err: GlError) -> Self {
        EngineError::GlError(err)
    }
}