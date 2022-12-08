use super::*;
use std::str::pattern::Pattern;

/// Parse the comment block
///
/// ```ygg
/// r#" "#
/// r##" "##
/// r###" "###
/// ```
pub fn surround_pair<S, E>(state: ParseState, start: S, end: E) -> ParseResult<&str>
where
    S: Pattern<'static>,
    E: Pattern<'static>,
{
    if !state.rest_text.starts_with(start) {
        StopBecause::missing_string(start, state.start_offset)?;
    }
    state.match_str()
    let mut offset = start.len();
    let rest = &state.rest_text[offset..];
    match rest.find(end) {
        Some(s) => offset += s + end.len(),
        None => StopBecause::missing_string(end, state.start_offset + offset)?,
    }
    state.advance_view(offset)
}
#[test]
fn test() {
    surround_pair(ParseState::new("r###\"hello\"###"), "r###", "###");
}

/// Parse the given state as a single quote string.
///
/// # Arguments
///
/// - `''`
/// - `""`
///
/// # Examples
///
/// ```
/// use pex::ParseState;
/// use yggdrasil_rt::helpers::paired_with_escaper;
/// paired_with_escaper(ParseState::new("'hello'"));
/// ```
pub fn surround_pair_with_escaper<'i>(state: ParseState<'i>, bound: char, escaper: char) -> ParseResult<&'i str> {
    let mut offset = 0;
    let mut rest = state.rest_text.chars().peekable();
    match rest.next() {
        Some(start) if start == bound => offset += bound.len_utf8(),
        _ => StopBecause::missing_character(bound, state.start_offset)?,
    }
    let mut closed = false;
    while let Some(c) = rest.next() {
        offset += c.len_utf8();
        match c {
            _ if c == bound => {
                closed = true;
                break;
            }
            _ if c == escaper => match rest.next() {
                Some(next) => offset += next.len_utf8(),
                None => StopBecause::missing_character_set("ANY", state.start_offset + offset)?,
            },
            _ => {}
        }
    }
    if !closed {
        StopBecause::missing_character(bound, state.start_offset + offset)?
    }
    state.advance_view(offset)
}

/// Parse the given state as a single quote string,
/// such strings are allowed to contain escape symbols `\\`,
/// if you want to disallow escape symbols, please use [ParseState::match_surround],
/// see more in examples
///
/// # Valid
///
/// - `''`
/// - `""`
///
/// # Examples
///
/// ```
/// # use pex::helpers::{single_quote_string, surround_pair};
/// # use pex::ParseState;
/// let normal = ParseState::new("'hello'");
/// let escape = ParseState::new("'hello \\\' world'");
///
/// assert!(single_quote_string(normal).is_success());
/// assert!(single_quote_string(escape).is_success());
/// assert!(surround_pair(normal, "\'", "\'").is_success());
/// assert!(surround_pair(escape, "\'", "\'").is_failure());
/// ```
pub fn single_quote_string<'i>(state: ParseState<'i>) -> ParseResult<&'i str> {
    surround_pair_with_escaper(state, '\'', '\\')
}

/// Parse the given state as a single quote string.
///
/// # Arguments
///
/// - `''`
/// - `""`
///
/// # Examples
///
/// ```
/// use pex::ParseState;
/// use yggdrasil_rt::helpers::double_quote_string;
/// double_quote_string(ParseState::new("'hello'"));
/// ```
pub fn double_quote_string<'i>(state: ParseState<'i>) -> ParseResult<&'i str> {
    surround_pair_with_escaper(state, '"', '\\')
}