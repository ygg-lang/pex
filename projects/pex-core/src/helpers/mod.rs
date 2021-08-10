use crate::{ParseResult, ParseState};

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
