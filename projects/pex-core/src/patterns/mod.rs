use std::{
    ops::Range,
    str::pattern::{Pattern, ReverseSearcher},
};

/// A pattern with a name
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
/// A pattern with a name
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

/// A pattern with a name
#[derive(Copy, Clone, Debug)]
pub struct SurroundPair<'i> {
    /// The length of the pattern
    pub lhs: StringView<'i>,
    /// The length of the pattern
    pub body: StringView<'i>,
    /// The length of the pattern
    pub rhs: StringView<'i>,
}

/// A pattern with a name
#[derive(Copy, Clone, Debug)]
pub struct SurroundPairPattern<S, E>
where
    S: Pattern<'static>,
    E: Pattern<'static>,
{
    pub lhs: NamedPattern<S>,
    pub rhs: NamedPattern<E>,
}

impl<S, E> SurroundPairPattern<S, E>
where
    S: Pattern<'static>,
    E: Pattern<'static>,
{
    /// Create a new named pattern
    pub fn new(start: NamedPattern<S>, end: NamedPattern<E>) -> Self {
        Self { lhs: start, rhs: end }
    }
}
