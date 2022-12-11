use super::*;

/// Used to parse matching surround pairs without escaping, often used to match raw strings,
/// such as `r###"TEXT"###` in rust and `"""TEXT"""` in toml.
///
/// For interpolated strings, it is recommended to use staged parsing, first match the original string,
/// then match the interpolation, [SurroundPair] contains the starting position information
///
/// ## Examples
///
/// ```ygg
/// r#" "#
/// r##" "##
/// r###" "###
/// ```
///
/// # Examples
///
/// - match `` `1234` ``
///
/// ```
/// # use pex::{helpers::surround_pair, NamedPattern, ParseState, SurroundPairPattern};
/// let quoted_str = SurroundPairPattern {
///     lhs: NamedPattern::new('`', "STRING_LHS"),
///     rhs: NamedPattern::new('`', "STRING_RHS"),
/// };
/// let test =
///     surround_pair(ParseState::new(r#"`12{x}34`rest text"#), quoted_str).as_result().unwrap().1;
/// assert_eq!(test.head.as_string(), "`");
/// assert_eq!(test.body.as_string(), "12{x}34");
/// assert_eq!(test.tail.as_string(), "`");
/// ```
///
/// - match `"""1234"""`
///
/// ```
/// # use pex::{helpers::surround_pair, NamedPattern, ParseState, SurroundPairPattern};
/// let raw_str = SurroundPairPattern {
///     lhs: NamedPattern::new("\"\"\"", "STRING_RAW_LHS"),
///     rhs: NamedPattern::new("\"\"\"", "STRING_RAW_RHS"),
/// };
/// let state = ParseState::new(r#""""1234"""rest text"#);
/// let paired = surround_pair(state, raw_str).as_result().unwrap().1;
/// assert_eq!(paired.head.as_string(), "\"\"\"");
/// assert_eq!(paired.body.as_string(), "1234");
/// assert_eq!(paired.tail.as_string(), "\"\"\"");
/// ```
pub fn surround_pair<'i, S, E>(state: ParseState<'i>, pattern: SurroundPairPattern<S, E>) -> ParseResult<'i, SurroundPair<'i>>
where
    S: Pattern<'static>,
    E: Pattern<'static>,
    'i: 'static,
{
    pattern.consume_state(state)
}

/// Parse the given state as a single quote string, all characters are allowed in strings except `'`, but including `\'`.
///
/// # Examples
pub fn surround_pair_with_escaper<'i>(state: ParseState<'i>, bound: char, escaper: char) -> ParseResult<&'i str> {
    let mut offset = 0;
    let mut rest = state.residual.chars().peekable();
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
/// if you want to disallow escape symbols, please use [surround_pair].
///
/// # Valid
///
/// - `''`
/// - `""`
///
/// # Examples
///
/// ```
/// # use pex::{helpers::{single_quote_string}, ParseState};
/// let normal = ParseState::new("'hello'");
/// let escape = ParseState::new("'hello \\\' world'");
///
/// assert!(single_quote_string(normal).is_success());
/// assert!(single_quote_string(escape).is_success());
/// ```
pub fn single_quote_string<'i>(state: ParseState<'i>) -> ParseResult<&'i str> {
    surround_pair_with_escaper(state, '\'', '\\')
}

/// Parse the given state as a single quote string,
/// such strings are allowed to contain escape symbols `\\`,
/// if you want to disallow escape symbols, please use [surround_pair].
///
///
/// # Examples
///
/// ```
/// # use pex::{helpers::{double_quote_string}, ParseState};
/// let normal = ParseState::new(r#""hello""#);
/// let escape = ParseState::new(r#""hello \" world""#);
///
/// assert!(double_quote_string(normal).is_success());
/// assert!(double_quote_string(escape).is_success());
/// ```
pub fn double_quote_string<'i>(state: ParseState<'i>) -> ParseResult<&'i str> {
    surround_pair_with_escaper(state, '"', '\\')
}
