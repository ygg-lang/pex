use crate::{
    helpers::{decimal_string, whitespace},
    NamedPattern, ParseResult, ParseState, StringView,
};
use alloc::vec::Vec;
use core::str::pattern::Pattern;

#[derive(Clone, Debug)]
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

impl<'p, P> BracketPattern<P>
where
    P: Pattern<'p> + Clone,
{
    /// ```js
    /// [ ~ ]
    /// [ ~ term (~ , ~ term)* (~ ,)? ~ ]
    ///
    /// <| a, b, c |>
    /// ```
    pub fn consume<'i, F, I, T, U>(&'p self, input: ParseState<'i>, ignore: I, parser: F) -> ParseResult<'i, BracketPair<'i, T>>
    where
        F: Fn(ParseState<'i>) -> ParseResult<'i, T> + Clone,
        I: Fn(ParseState<'i>) -> ParseResult<'i, U> + Clone,
        'i: 'p,
    {
        input
            .begin_choice()
            .or_else(|s| self.maybe_empty(s, ignore.clone()))
            .or_else(|s| self.maybe_many(s, ignore.clone(), parser.clone()))
            .end_choice()
    }

    /// `[ ~ ]`
    fn maybe_empty<'i, I, T, U>(&'p self, input: ParseState<'i>, ignore: I) -> ParseResult<'i, BracketPair<'i, T>>
    where
        I: Fn(ParseState<'i>) -> ParseResult<'i, U>,
        'i: 'p,
    {
        let (state, lhs) = self.open.consume(input)?;
        let (state, rhs) = self.close.consume(input.skip(ignore))?;
        state.finish(BracketPair { lhs, rhs, body: Vec::new() })
    }
    /// `[ ~ term (~ , ~ term)* ~ ,? ~ ]`
    fn maybe_many<'i, F, I, T, U>(&'p self, input: ParseState<'i>, ignore: I, parser: F) -> ParseResult<'i, BracketPair<'i, T>>
    where
        F: Fn(ParseState<'i>) -> ParseResult<'i, T> + Clone,
        I: Fn(ParseState<'i>) -> ParseResult<'i, U> + Clone,
        'i: 'p,
    {
        let mut out = Vec::with_capacity(1);
        let (state, lhs) = self.open.consume(input)?;
        let (state, first) = state.skip(&ignore).match_fn(&parser)?;
        out.push(first);
        let (state, _) = state.match_repeats(|s| self.delimiter_term(s, ignore.clone(), parser.clone(), &mut out))?;

        todo!()
    }
    /// `~ , ~ term`
    fn delimiter_term<'i, 't, F, I, T, U>(
        &'p self,
        input: ParseState<'i>,
        ignore: I,
        parser: F,
        terms: &'t mut Vec<T>,
    ) -> ParseResult<'i, ()>
    where
        F: Fn(ParseState<'i>) -> ParseResult<'i, T>,
        I: Fn(ParseState<'i>) -> ParseResult<'i, U>,
        'i: 'p,
    {
        let (state, _) = self.delimiter.consume(input.skip(ignore))?;
        let (state, term) = parser(state)?;
        terms.push(term);
        state.finish(())
    }
}
