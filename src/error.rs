use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct CustomError {
    message: String,
}

impl CustomError {
    pub fn new<T: AsRef<str>>(message: T) -> Self {
        CustomError {
            message: message.as_ref().to_owned(),
        }
    }
}

impl std::error::Error for CustomError {}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
