use super::*;
use crate::helpers::{decimal_string, whitespace};
use alloc::vec::Vec;

pub struct BracketPattern<P> {
    pub open: NamedPattern<P>,
    pub close: NamedPattern<P>,
    pub delimiter: NamedPattern<P>,
    pub dangling: Option<bool>,
}

#[derive(Debug)]
pub struct BracketPair<'i, T> {
    lhs: StringView<'i>,
    rhs: StringView<'i>,
    body: Vec<T>,
}

impl<P> BracketPattern<P>
where
    P: Pattern<'static>,
{
    /// ```js
    /// [ ~ ]
    /// [ ~ term (~ , ~ term)* (~ ,)? ~ ]
    /// ```
    pub fn consume<'i, F, I, T, U>(&self, input: ParseState<'i>, ignore: I, parser: F) -> ParseResult<'i, BracketPair<'i, T>>
    where
        F: Fn(ParseState<'i>) -> ParseResult<'i, T>,
        I: Fn(ParseState<'i>) -> ParseResult<'i, U>,
    {
        todo!()
    }
}

pub fn maybe_empty<'i, I, U, P, T>(
    input: ParseState<'i>,
    ignore: I,
    config: &BracketPattern<P>,
) -> ParseResult<'i, BracketPair<'i, T>>
where
    I: Fn(ParseState<'i>) -> ParseResult<'i, U>,
{
    let (state, _) = input.match_str_pattern(&config.open.pattern, config.open.message)?;
    todo!()
}
