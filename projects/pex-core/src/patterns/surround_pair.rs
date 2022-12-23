use super::*;
use crate::{ParseResult, ParseState, StopBecause};
use core::str::pattern::Searcher;

/// Represents a three-segment string match, including the `head`, `body`, and `tail`, see more example in
/// [surround_pair](crate::helpers::surround_pair),
/// [double_quote_string](crate::helpers::escaped_quotation_pair),
/// [single_quote_string](crate::helpers::single_quote_string)
#[derive(Copy, Clone, Debug)]
pub struct SurroundPair<'i> {
    /// The length of the pattern
    pub head: StringView<'i>,
    /// The length of the pattern
    pub body: StringView<'i>,
    /// The length of the pattern
    pub tail: StringView<'i>,
}

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
/// let test =
///     surround_pair(ParseState::new(r#""""1234"""rest text"#), raw_str).as_result().unwrap().1;
/// assert_eq!(test.head.as_string(), "\"\"\"");
/// assert_eq!(test.body.as_string(), "1234");
/// assert_eq!(test.tail.as_string(), "\"\"\"");
/// ```
#[derive(Copy, Clone, Debug)]
pub struct SurroundPairPattern<S, E>
where
    S: Pattern<'static>,
    E: Pattern<'static>,
{
    /// The length of the pattern
    pub lhs: NamedPattern<S>,
    /// The length of the pattern
    pub rhs: NamedPattern<E>,
}

impl<S, E> SurroundPairPattern<S, E>
where
    S: Pattern<'static>,
    E: Pattern<'static>,
{
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
    /// let template_str = SurroundPairPattern {
    ///     lhs: NamedPattern::new('`', "STRING_LHS"),
    ///     rhs: NamedPattern::new('`', "STRING_RHS"),
    /// };
    /// let raw_str = SurroundPairPattern {
    ///     lhs: NamedPattern::new("\"\"\"", "STRING_RAW_LHS"),
    ///     rhs: NamedPattern::new("\"\"\"", "STRING_RAW_RHS"),
    /// };
    /// ```
    pub fn new(start: NamedPattern<S>, end: NamedPattern<E>) -> Self {
        Self { lhs: start, rhs: end }
    }
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
    /// let template_str = SurroundPairPattern {
    ///     lhs: NamedPattern::new('`', "STRING_LHS"),
    ///     rhs: NamedPattern::new('`', "STRING_RHS"),
    /// };
    /// let raw_str = SurroundPairPattern {
    ///     lhs: NamedPattern::new("\"\"\"", "STRING_RAW_LHS"),
    ///     rhs: NamedPattern::new("\"\"\"", "STRING_RAW_RHS"),
    /// };
    /// ```
    pub fn consume_state<'i>(self, state: ParseState<'i>) -> ParseResult<'i, SurroundPair<'i>>
    where
        'i: 'static,
    {
        let (body_state, head) = state.match_str_pattern(self.lhs.pattern, self.lhs.message)?;
        let lhs = StringView::new(head, state.start_offset);
        let message = self.rhs.message;
        let mut searcher = self.rhs.into_searcher(&body_state.residual);
        match searcher.next_match() {
            // SAFETY: the searcher is guaranteed to return valid indices
            Some((start, end)) => unsafe {
                let body_str = body_state.residual.get_unchecked(..start);
                let rhs_str = body_state.residual.get_unchecked(start..end);
                let body = StringView::new(body_str, body_state.start_offset);
                let rhs = StringView::new(rhs_str, body_state.start_offset + start);
                let new_state = body_state.advance(end);
                new_state.finish(SurroundPair { head: lhs, body, tail: rhs })
            },
            None => StopBecause::missing_string(message, body_state.end_offset())?,
        }
    }
}
