#![doc = include_str!("readme.md")]
use crate::{ParseResult, ParseState, StopBecause, SurroundPair, SurroundPairPattern};
mod bracket;
mod color;
mod comment;
mod number;
mod string;
pub use self::{
    color::hex_color,
    comment::{comment_block, comment_block_nested, comment_line},
    number::*,
    string::{quotation_pair, quotation_pair_escaped, quotation_pair_nested, surround_pair, surround_pair_with_escaper},
};
use crate::ParseResult::{Pending, Stop};
use core::str::pattern::Pattern;

/// Match ascii whitespace and newlines, fail if empty
///
/// # Examples
///
/// ```
/// # use pex::{ParseResult, ParseState, helpers::ascii_whitespace};
/// let state = ParseState::new("  \na");
/// state.skip(ascii_whitespace);
/// ```
#[inline]
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
#[inline]
pub fn whitespace<'i>(state: ParseState<'i>) -> ParseResult<&'i str> {
    match state.residual.find(|c: char| !c.is_whitespace()) {
        Some(len) => state.advance_view(len),
        None => StopBecause::missing_character(' ', state.start_offset)?,
    }
}

/// Function form of the str combinator.
///
/// # Examples
///
/// ```
/// # use pex::{ParseResult, ParseState, helpers::str};
/// let state = ParseState::new("  \na");
/// state.skip(str(" "));
/// ```
#[inline]
pub fn str<'i>(s: &'static str) -> impl Fn(ParseState<'i>) -> ParseResult<'i, &'i str> {
    move |input: ParseState| input.match_str(s)
}

/// Function form of the char combinator.
///
/// # Examples
///
/// ```
/// # use pex::{ParseResult, ParseState, helpers::char};
/// let state = ParseState::new("  \na");
/// state.skip(char(' '));
/// ```
#[inline]
pub fn char(c: char) -> impl Fn(ParseState) -> ParseResult<char> {
    move |input: ParseState| input.match_char(c)
}

/// Function form of the char combinator.
///
/// # Examples
///
/// ```
/// # use pex::{ParseResult, ParseState, helpers::{char, omit}};
/// let state = ParseState::new("  \na");
/// state.skip(omit(char(' ')));
/// ```
#[inline]
pub fn omit<T, F>(parse: F) -> impl Fn(ParseState) -> ParseResult<()>
where
    F: Fn(ParseState) -> ParseResult<T>,
{
    move |input: ParseState| parse(input).map_inner(|_| ())
}

/// Function form of the optional combinator.
///
/// # Examples
#[inline]
pub fn optional<T, F>(parse: F) -> impl Fn(ParseState) -> ParseResult<Option<T>>
where
    F: Fn(ParseState) -> ParseResult<T>,
{
    move |input: ParseState| match parse(input) {
        Pending(state, value) => state.finish(Some(value)),
        Stop(_) => input.finish(None),
    }
}

/// Make the [`from_str`](core::str::FromStr) function from the pex parser
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
#[inline]
pub fn make_from_str<T, F>(state: ParseState, parser: F) -> Result<T, StopBecause>
where
    F: FnOnce(ParseState) -> ParseResult<T>,
{
    match parser(state) {
        ParseResult::Pending(state, compound) if state.is_empty() => Ok(compound),
        ParseResult::Pending(state, ..) => Err(StopBecause::ExpectEOF { position: state.start_offset }),
        ParseResult::Stop(e) => Err(e),
    }
}
