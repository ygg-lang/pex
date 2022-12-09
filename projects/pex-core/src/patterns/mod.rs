use std::str::pattern::{Pattern, ReverseSearcher};

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
