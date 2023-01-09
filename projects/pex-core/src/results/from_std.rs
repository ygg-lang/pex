use crate::{CustomError, StopBecause};
use core::{
    char::CharTryFromError,
    num::{ParseFloatError, ParseIntError},
};

impl From<ParseFloatError> for StopBecause {
    fn from(_: ParseFloatError) -> Self {
        CustomError { message: "Invalid float literal", start: 0, end: 0 }.into()
    }
}

impl From<ParseIntError> for StopBecause {
    fn from(_: ParseIntError) -> Self {
        CustomError { message: "Invalid integer literal", start: 0, end: 0 }.into()
    }
}

impl From<CharTryFromError> for StopBecause {
    fn from(_: CharTryFromError) -> Self {
        CustomError { message: "Character out of range", start: 0, end: 0 }.into()
    }
}
