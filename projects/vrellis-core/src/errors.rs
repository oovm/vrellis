use image::ImageError;

#[derive(Debug, Clone)]
pub enum VrellisError {
    NoneError,
    IOError(String),
    ImageError(String),
    SerdeError(String),
}

pub type Result<T> = std::result::Result<T, VrellisError>;

impl From<std::io::Error> for VrellisError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(format!("{}", e))
    }
}

impl From<ImageError> for VrellisError {
    fn from(e: ImageError) -> Self {
        match e {
            ImageError::IoError(e) => Self::IOError(format!("{}", e)),
            _ => Self::ImageError(format!("{}", e)),
        }
    }
}
