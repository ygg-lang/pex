use std::{
    convert::Infallible,
    error::Error,
    fmt::{Debug, Display, Formatter},
    ops::{ControlFlow, FromResidual, Range, Try},
};

use crate::{ParseState, Parsed};

mod residual;

mod methods;
mod reason;

/// Represent as parsing result
#[derive(Eq, PartialEq)]
pub enum ParseResult<'i, T> {
    /// The parsing is not finished yet
    Pending(ParseState<'i>, T),
    /// The parsing is finished, and give the reason why
    Stop(StopBecause),
}

/// Must copy
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum StopBecause {
    /// This error is not initialized
    Uninitialized,
    /// Expect end of string
    ExpectEof {
        /// The offset of the location where the error occurred
        position: usize,
    },
    /// Expect repeats pattern
    ExpectRepeats {
        /// The minimum of repeats
        min: usize,
        /// The maximum of repeats
        current: usize,
        /// The offset of the location where the error occurred
        position: usize,
    },
    /// Expect some character or character range in range
    MissingCharacterRange {
        /// The start of the range
        start: char,
        /// The end of the range
        end: char,
        /// The offset of the location where the error occurred
        position: usize,
    },
    /// Expect some character
    MissingCharacterSet {
        /// The expected character
        expected: &'static str,
        /// The offset of the location where the error occurred
        position: usize,
    },
    /// Expect some string
    MissingString {
        /// The error message
        message: &'static str,
        /// The offset of the location where the error occurred
        position: usize,
    },
    /// Must be some pattern
    MustBe {
        /// The error message
        message: &'static str,
        /// The offset of the location where the error occurred
        position: usize,
    },
    /// Should not be some pattern
    ShouldNotBe {
        /// The error message
        message: &'static str,
        /// The offset of the location where the error occurred
        position: usize,
    },
    /// A custom error message
    Custom(CustomError),
}

/// A custom error message
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CustomError {
    /// The error message
    pub message: &'static str,
    /// The start offset of the location where the error occurred
    pub start: usize,
    /// The end offset of the location where the error occurred
    pub end: usize,
}

impl From<CustomError> for StopBecause {
    fn from(value: CustomError) -> Self {
        Self::Custom(value)
    }
}
