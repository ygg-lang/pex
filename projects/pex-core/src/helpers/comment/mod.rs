use super::*;
use crate::{NamedPattern, StringView};

/// Parse a comment line, note this does not catch the newline,
/// catch all if reach [EOF]()
///
/// # Patterns
///
/// ```regex
/// # comment
/// // comment
/// ```
///
/// # Examples
///
/// ```
/// # use pex::{helpers::comment_line, ParseState};
/// let test1 = ParseState::new("# comment hash\r text text");
/// let test2 = ParseState::new("// comment slash\n 123456");
/// assert_eq!(comment_line(test1, "#").unwrap().body.as_string(), " comment hash");
/// assert_eq!(comment_line(test2, "//").unwrap().body.as_string(), " comment slash");
/// ```
pub fn comment_line<'i>(state: ParseState<'i>, head: &'static str) -> ParseResult<'i, SurroundPair<'i>> {
    let (_, head) = state.match_str(head)?;
    let offset = match state.residual.find(&['\r', '\n']) {
        Some(s) => s,
        None => state.residual.len(),
    };
    // SAFETY: find offset always valid
    let body = unsafe { state.residual.get_unchecked(head.len()..offset) };
    state.advance(offset).finish(SurroundPair {
        head: StringView::new(head, state.start_offset),
        body: StringView::new(body, state.start_offset + head.len()),
        tail: StringView::new("", state.start_offset + offset),
    })
}

/// Parse the comment block
///
/// # Patterns
///
/// ```regex
/// /** comment */
/// (* comment *)
/// ```
///
/// # Examples
///
/// ```
/// # use pex::{helpers::comment_block, ParseState};
/// let test1 = ParseState::new("(*  comment  *) 123456");
/// let test2 = ParseState::new("/** comment **/ 123456");
/// assert_eq!(comment_block(test1, "(*", "*)").unwrap().body.as_string(), "  comment  ");
/// assert_eq!(comment_block(test2, "/*", "*/").unwrap().body.as_string(), "* comment *");
/// ```
pub fn comment_block<'i>(state: ParseState<'i>, head: &'static str, tail: &'static str) -> ParseResult<'i, SurroundPair<'i>>
where
    'i: 'static,
{
    let raw_str = SurroundPairPattern {
        //
        lhs: NamedPattern::new(head, "COMMENT_LHS"),
        rhs: NamedPattern::new(tail, "COMMENT_RHS"),
    };
    surround_pair(state, raw_str)
}

/// Comment a nested block
pub fn comment_block_nested<'i>(
    state: ParseState<'i>,
    head: &'static str,
    tail: &'static str,
) -> ParseResult<'i, SurroundPair<'i>> {
    let (body_state, head) = state.match_str(head)?;
    let mut nested = 1;
}
