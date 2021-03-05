use image;
use std::io;
use std::fmt;

pub enum ImageError {
    Io(io::Error),
    Image(image::ImageError)
}

impl From<io::Error> for ImageError {
    fn from (e: io::Error) -> Self {
        ImageError::Io(e)
    }
}

impl From<image::ImageError> for ImageError {
    fn from (e: image::ImageError) -> Self {
        ImageError::Image(e)
    }
}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ImageError::Io(e) => write!(f, "{}", e),
            ImageError::Image(e) => write!(f, "{}", e)
        }
    }
}

pub struct UnknownCharacterError {
    pub character: String,
}

impl fmt::Display for UnknownCharacterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unknown Character Error: {:?}", self.character)
    }
}

pub enum TranslationError {
    UnknownCharacter(UnknownCharacterError)
}

impl From<UnknownCharacterError> for TranslationError {
    fn from (e: UnknownCharacterError) -> Self {
        TranslationError::UnknownCharacter(e)
    }
}


impl fmt::Display for TranslationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TranslationError::UnknownCharacter(e) => write!(f, "{}", e)
        }
    }
}
