use crate::{NamedPattern, ParseResult, ParseState, StringView};
use alloc::vec::Vec;
use core::str::pattern::Pattern;

#[derive(Clone, Debug)]
pub struct BracketPattern {
    pub open: &'static str,
    pub close: &'static str,
    pub delimiter: &'static str,
    pub dangling: Option<bool>,
}

#[derive(Debug)]
pub struct BracketPair<'i, T> {
    lhs: StringView<'i>,
    rhs: StringView<'i>,
    body: Vec<T>,
}

impl BracketPattern {
    /// ```js
    /// [ ~ ]
    /// [ ~ term (~ , ~ term)* (~ ,)? ~ ]
    ///
    /// <| a, b, c |>
    /// ```
    pub fn consume<'i, F, I, T, U>(&self, input: ParseState<'i>, ignore: I, parser: F) -> ParseResult<'i, BracketPair<'i, T>>
    where
        F: Fn(ParseState<'i>) -> ParseResult<'i, T>,
        I: Fn(ParseState<'i>) -> ParseResult<'i, U>,
    {
        input
            .begin_choice()
            .or_else(|s| self.maybe_empty(s, &ignore))
            .or_else(|s| self.maybe_many(s, &ignore, &parser))
            .end_choice()
    }

    /// `[ ~ ]`
    fn maybe_empty<'i, I, T, U>(&self, input: ParseState<'i>, ignore: I) -> ParseResult<'i, BracketPair<'i, T>>
    where
        I: Fn(ParseState<'i>) -> ParseResult<'i, U>,
    {
        let (state, lhs) = input.match_str(self.open)?;
        let (state, rhs) = input.skip(ignore).match_str(self.close)?;
        state.finish(BracketPair { lhs: StringView::new(lhs), rhs: StringView::new(rhs), body: Vec::new() })
    }
    /// `[ ~ term (~ , ~ term)* ~ ,? ~ ]`
    fn maybe_many<'i, F, I, T, U>(&self, input: ParseState<'i>, ignore: I, parser: F) -> ParseResult<'i, BracketPair<'i, T>>
    where
        F: Fn(ParseState<'i>) -> ParseResult<'i, T>,
        I: Fn(ParseState<'i>) -> ParseResult<'i, U>,
    {
        let mut terms = Vec::with_capacity(1);
        let (state, lhs) = self.open.consume(input)?;
        let (state, first) = state.skip(&ignore).match_fn(&parser)?;
        terms.push(first);
        let (state, _) = state.match_repeats(|s| self.delimiter_term(s, &ignore, &parser, &mut terms))?;
        let state = match self.dangling {
            Some(true) => self.delimiter.consume(state.skip(&ignore))?.0,
            Some(false) => state,
            None => match self.delimiter.consume(state.skip(&ignore)) {
                ParseResult::Pending(s, _) => s,
                ParseResult::Stop(_) => state,
            },
        };
        let (state, rhs) = self.close.consume(input.skip(&ignore))?;
        state.finish(BracketPair { lhs, rhs, body: terms })
    }
    /// `~ , ~ term`
    fn delimiter_term<'i, 't, F, I, T, U>(
        self,
        input: ParseState<'i>,
        ignore: I,
        parser: F,
        terms: &'t mut Vec<T>,
    ) -> ParseResult<'i, ()>
    where
        F: Fn(ParseState<'i>) -> ParseResult<'i, T>,
        I: Fn(ParseState<'i>) -> ParseResult<'i, U>,
    {
        let (state, _) = self.delimiter.consume(input.skip(ignore))?;
        let (state, term) = parser(state)?;
        terms.push(term);
        state.finish(())
    }
}
