#![doc = include_str!("readme.md")]

use crate::{ParseResult, ParseState, StopBecause};
use std::str::FromStr;
mod number;

pub use self::number::*;

/// Match ascii whitespace and newlines, never fails
///
/// # Examples
///
/// ```no_run
/// # use pex::{ParseResult, ParseState};
/// # use pex::helpers::ascii_whitespace;
/// let state = ParseState::new("  \na");
/// state.skip(ascii_whitespace);
/// ```
pub fn ascii_whitespace(state: ParseState) -> ParseResult<Box<str>> {
    let len = state.rest_text.find(|c: char| !c.is_ascii_whitespace()).unwrap_or(0);
    state.advance_view(len).map_inner(|s| Box::from(s))
}

/// Match whitespace and newlines, never fails
///
/// # Examples
///
/// ```no_run
/// # use pex::{ParseResult, ParseState};
/// # use pex::helpers::whitespace;
/// let state = ParseState::new("  \na");
/// state.skip(whitespace);
/// ```
pub fn whitespace(state: ParseState) -> ParseResult<Box<str>> {
    let len = state.rest_text.find(|c: char| !c.is_whitespace()).unwrap_or(0);
    state.advance_view(len).map_inner(|s| Box::from(s))
}

/// Make the [`from_str`](std::str::FromStr) function from state parser
///
/// # Arguments
///
/// * `state`:
/// * `parser`:
///
/// returns: Result<T, StopBecause>
///
/// # Examples
///
/// ```no_run
/// # use std::str::FromStr;
/// # use pex::{helpers::{make_from_str, whitespace}, ParseState, StopBecause};
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
