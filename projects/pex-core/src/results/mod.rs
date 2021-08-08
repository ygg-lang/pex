use std::{
    convert::Infallible,
    error::Error,
    fmt::{Display, Formatter},
    ops::{ControlFlow, FromResidual, Range, Try},
};

use crate::{ParseState, Parsed};

mod residual;

mod methods;
mod reason;

/// Represent as parsing result
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
    /// Expect some character
    MissingCharacter {
        /// The expected character
        expected: char,
        /// The offset of the location where the error occurred
        position: usize,
    },
    /// Expect some character in range
    MissingCharacterRange {
        /// The start of the range
        start: char,
        /// The end of the range
        end: char,
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
    /// Custom error
    Custom {
        /// The error message
        message: &'static str,
        /// The offset of the location where the error occurred
        position: usize,
    },
}
