use std::fmt;

#[derive(Debug, Default, Clone, Copy)]
pub enum DisplayErrorCodes {
    #[default]
    InvalidBrightnessValueError,
    InvalidBrightnessPathError,
}

//impl fmt::Display  for DisplayErrorCodes
impl std::fmt::Display for DisplayErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisplayErrorCodes::InvalidBrightnessValueError => {
                write!(f, "InvalidBrightnessValueError")
            }
            DisplayErrorCodes::InvalidBrightnessPathError => {
                write!(f, "InvalidBrightnessPathError")
            }
        }
    }
}

#[derive(Debug)]
pub struct DisplayError {
    pub code: DisplayErrorCodes,
    pub message: String,
}

//impl fmt::Display for DisplayError
impl std::fmt::Display for DisplayError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}

impl DisplayError {
    pub fn new(code: DisplayErrorCodes, message: String) -> Self {
        DisplayError { code, message }
    }
}
