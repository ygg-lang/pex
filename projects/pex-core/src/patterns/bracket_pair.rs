use crate::{ParseResult, ParseState, StringView};
use alloc::vec::Vec;

///
#[derive(Copy, Clone, Debug)]
pub struct BracketPattern {
    /// The
    pub open: &'static str,
    ///
    pub close: &'static str,
    ///
    pub delimiter: &'static str,
    ///
    pub dangling: Option<bool>,
}

///
#[derive(Debug)]
pub struct BracketPair<'i, T> {
    ///
    pub lhs: StringView<'i>,
    ///
    pub rhs: StringView<'i>,
    ///
    pub body: Vec<T>,
}

impl BracketPattern {
    /// Create a new bracket pattern
    pub fn new(open: &'static str, close: &'static str) -> Self {
        Self { open, close, delimiter: ",", dangling: None }
    }
    /// Create a new bracket pattern
    pub fn with_delimiter(mut self, delimiter: &'static str) -> Self {
        self.delimiter = delimiter;
        self
    }
    /// Create a new bracket pattern
    pub fn with_dangling(mut self, dangling: bool) -> Self {
        self.dangling = Some(dangling);
        self
    }
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
        let (s_rhs, lhs) = input.match_str(self.open)?;
        let (finally, rhs) = s_rhs.skip(ignore).match_str(self.close)?;
        finally.finish(BracketPair {
            lhs: StringView::new(lhs, input.start_offset),
            rhs: StringView::new(rhs, s_rhs.start_offset),
            body: Vec::new(),
        })
    }
    /// `[ ~ term (~ , ~ term)* ~ ,? ~ ]`
    fn maybe_many<'i, F, I, T, U>(&self, input: ParseState<'i>, ignore: I, parser: F) -> ParseResult<'i, BracketPair<'i, T>>
    where
        F: Fn(ParseState<'i>) -> ParseResult<'i, T>,
        I: Fn(ParseState<'i>) -> ParseResult<'i, U>,
    {
        let mut terms = Vec::with_capacity(1);
        let (state, lhs) = input.match_str(self.open)?;
        let (state, first) = state.skip(&ignore).match_fn(&parser)?;
        terms.push(first);
        let (state, _) = state.match_repeats(|s| self.delimiter_term(s, &ignore, &parser, &mut terms))?;
        let s_rhs = match self.dangling {
            Some(true) => state.skip(&ignore).match_str(self.delimiter)?.0,
            Some(false) => state,
            None => match state.skip(&ignore).match_str(self.delimiter) {
                ParseResult::Pending(s, _) => s,
                ParseResult::Stop(_) => state,
            },
        };

        let (finally, rhs) = s_rhs.skip(&ignore).match_str(self.close)?;
        finally.finish(BracketPair {
            lhs: StringView::new(lhs, input.start_offset),
            rhs: StringView::new(rhs, s_rhs.start_offset),
            body: terms,
        })
    }
    /// `~ , ~ term`
    fn delimiter_term<'i, 't, F, I, T, U>(
        &self,
        input: ParseState<'i>,
        ignore: I,
        parser: F,
        terms: &'t mut Vec<T>,
    ) -> ParseResult<'i, ()>
    where
        F: Fn(ParseState<'i>) -> ParseResult<'i, T>,
        I: Fn(ParseState<'i>) -> ParseResult<'i, U>,
    {
        let (state, _) = input.skip(&ignore).match_str(self.delimiter)?;
        let (state, term) = state.skip(&ignore).match_fn(parser)?;
        terms.push(term);
        state.finish(())
    }
}
