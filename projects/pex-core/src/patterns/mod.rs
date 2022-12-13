use alloc::{borrow::ToOwned, string::String};
use core::{
    ops::Range,
    str::pattern::{Pattern, ReverseSearcher},
};
pub mod surround_pair;

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
pub struct NamedPattern<P>
where
    P: Pattern<'static>,
{
    /// The pattern to match
    pub pattern: P,
    /// Error message to display if the pattern is failed to match
    pub message: &'static str,
}

impl<P> Pattern<'static> for NamedPattern<P>
where
    P: Pattern<'static>,
{
    type Searcher = P::Searcher;

    fn into_searcher(self, haystack: &'static str) -> Self::Searcher {
        self.pattern.into_searcher(haystack)
    }

    fn is_contained_in(self, haystack: &'static str) -> bool {
        self.pattern.is_contained_in(haystack)
    }

    fn is_prefix_of(self, haystack: &'static str) -> bool {
        self.pattern.is_prefix_of(haystack)
    }

    fn is_suffix_of(self, haystack: &'static str) -> bool
    where
        Self::Searcher: ReverseSearcher<'static>,
    {
        self.pattern.is_suffix_of(haystack)
    }

    fn strip_prefix_of(self, haystack: &'static str) -> Option<&'static str> {
        self.pattern.strip_prefix_of(haystack)
    }

    fn strip_suffix_of(self, haystack: &'static str) -> Option<&'static str>
    where
        Self::Searcher: ReverseSearcher<'static>,
    {
        self.pattern.strip_suffix_of(haystack)
    }
}

impl<P> NamedPattern<P>
where
    P: Pattern<'static>,
{
    /// Create a new named pattern
    pub fn new(pattern: P, message: &'static str) -> Self {
        Self { pattern, message }
    }
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
pub struct StringView<'i> {
    start_offset: usize,
    string: &'i str,
}

impl<'i> AsRef<str> for StringView<'i> {
    fn as_ref(&self) -> &str {
        self.string
    }
}

impl<'i> StringView<'i> {
    /// Create a new named pattern
    pub fn new(string: &'i str, start_offset: usize) -> Self {
        Self { start_offset, string }
    }
    /// Create a new named pattern
    pub fn start_offset(&self) -> usize {
        self.start_offset
    }
    /// Create a new named pattern
    pub fn end_offset(&self) -> usize {
        self.start_offset + self.string.len()
    }
    /// Create a new named pattern
    pub fn as_range(&self) -> Range<usize> {
        self.start_offset..self.end_offset()
    }
    /// Create a new named pattern
    pub fn as_string(&self) -> String {
        self.string.to_owned()
    }
}
