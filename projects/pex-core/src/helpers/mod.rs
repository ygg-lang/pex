#![doc = include_str!("readme.md")]

use crate::{ParseResult, ParseState, StopBecause};
use std::str::FromStr;

mod color;
mod number;
mod string;

pub use self::{
    color::hex_color,
    number::*,
    string::{double_quote_string, single_quote_string, surround_pair, surround_pair_with_escaper},
};

/// Match ascii whitespace and newlines, fail if empty
///
/// # Examples
///
/// ```
/// # use pex::{ParseResult, ParseState, helpers::ascii_whitespace};
/// let state = ParseState::new("  \na");
/// state.skip(ascii_whitespace);
/// ```
pub fn ascii_whitespace<'i>(state: ParseState<'i>) -> ParseResult<&'i str> {
    match state.residual.find(|c: char| !c.is_ascii_whitespace()) {
        Some(len) => state.advance_view(len),
        None => StopBecause::missing_character(' ', state.start_offset)?,
    }
}

/// Match whitespace and newlines, fail if empty
///
/// # Examples
///
/// ```
/// # use pex::{ParseResult, ParseState, helpers::whitespace};
/// let state = ParseState::new("  \na");
/// state.skip(whitespace);
/// ```
pub fn whitespace<'i>(state: ParseState<'i>) -> ParseResult<&'i str> {
    match state.residual.find(|c: char| !c.is_whitespace()) {
        Some(len) => state.advance_view(len),
        None => StopBecause::missing_character(' ', state.start_offset)?,
    }
}

/// Make the [`from_str`](std::str::FromStr) function from the pex parser
///
/// # Examples
///
/// ```
/// # use std::str::FromStr;
/// # use pex::{helpers::{make_from_str, whitespace}, ParseResult, ParseState, StopBecause};
/// # struct Compound;
/// # impl Compound {
/// #     fn parse(state: ParseState) -> ParseResult<Self> {
/// #         state.finish(Compound)
/// #     }
/// # }
/// impl FromStr for Compound {
///     type Err = StopBecause;
///
///     fn from_str(s: &str) -> Result<Self, StopBecause> {
///         // ignore whitespace at the start and end
///         let state = ParseState::new(s.trim_end()).skip(whitespace);
///         make_from_str(state, Self::parse)
///     }
/// }
/// ```
pub fn make_from_str<T, F>(state: ParseState, parser: F) -> Result<T, StopBecause>
where
    F: FnOnce(ParseState) -> ParseResult<T>,
{
    match parser(state) {
        ParseResult::Pending(state, compound) if state.is_empty() => Ok(compound),
        ParseResult::Pending(state, ..) => Err(StopBecause::ExpectEof { position: state.start_offset }),
        ParseResult::Stop(e) => Err(e),
    }
}
