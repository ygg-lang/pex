use super::*;
use crate::{ParseResult, ParseState, StopBecause};
use core::str::pattern::{Pattern, Searcher};

/// Represents a three-segment string match, including the `head`, `body`, and `tail`, see more example in
/// [surround_pair](crate::helpers::surround_pair),
/// [double_quote_string](crate::helpers::quotation_pair_escaped),
/// [single_quote_string](crate::helpers::quotation_pair)
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
/// # use pex::{NamedPattern, ParseState, helpers::SurroundPattern};
/// let quoted_str =
///     SurroundPattern { lhs: "`", rhs: "`", lhs_name: "TEMPLATE_LHS", rhs_name: "TEMPLATE_RHS" };
/// let test = quoted_str(ParseState::new(r#"`12{x}34`rest text"#)).unwrap();
/// assert_eq!(test.head.as_string(), "`");
/// assert_eq!(test.body.as_string(), "12{x}34");
/// assert_eq!(test.tail.as_string(), "`");
/// ```
///
/// - match `"""1234"""`
///
/// ```
/// # use pex::{NamedPattern, ParseState, helpers::SurroundPattern};
/// let raw_str = SurroundPattern {
///     lhs: "\"\"\"",
///     rhs: "\"\"\"",
///     lhs_name: "STRING_RAW_LHS",
///     rhs_name: "STRING_RAW_RHS",
/// };
/// let test = raw_str(ParseState::new(r#""""1234"""rest text"#)).unwrap();
/// assert_eq!(test.head.as_string(), "\"\"\"");
/// assert_eq!(test.body.as_string(), "1234");
/// assert_eq!(test.tail.as_string(), "\"\"\"");
/// ```
#[derive(Copy, Clone, Debug)]
pub struct SurroundPattern {
    /// The start pattern
    pub lhs: &'static str,
    /// The end pattern
    pub rhs: &'static str,
    /// The name of the start pattern
    pub lhs_name: &'static str,
    /// The name of the end pattern
    pub rhs_name: &'static str,
}

impl<'i> FnOnce<(ParseState<'i>,)> for SurroundPattern {
    type Output = ParseResult<'i, SurroundPair<'i>>;

    #[inline]
    extern "rust-call" fn call_once(self, (input,): (ParseState<'i>,)) -> Self::Output {
        let (body_state, head) = input.match_str_pattern(self.lhs, self.lhs_name)?;
        let lhs = StringView::new(head, input.start_offset);
        let message = self.rhs_name;
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
