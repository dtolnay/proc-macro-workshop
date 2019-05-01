use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Error {
    typename: &'static str,
    value: u64,
    width: u8,
}

impl Error {
    #[doc(hidden)]
    pub fn new(typename: &'static str, value: u64, width: u8) -> Error {
        Error {
            typename,
            value,
            width,
        }
    }

    pub fn raw_value(&self) -> u64 {
        self.value
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "unrecognized bit pattern for enum {}: 0b{:02$b}",
            self.typename, self.value, self.width as usize,
        )
    }
}

impl std::error::Error for Error {}
