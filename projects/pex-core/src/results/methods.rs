use std::fmt::Debug;

use super::*;

impl<'i, T> Debug for ParseResult<'i, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseResult::Pending(s, v) => f
                .debug_struct("Pending")
                .field("value", v)
                .field("rest_text", &s.rest_text)
                .field("start_offset", &s.start_offset)
                .field("stop_reason", &s.stop_reason)
                .finish(),
            ParseResult::Stop(e) => f.debug_struct("Stop").field("reason", e).finish(),
        }
    }
}

impl<'i, T> ParseResult<'i, T> {
    /// Map inner value
    ///
    /// ```
    /// # use pex::{ParseResult, ParseState};
    /// let state = ParseState::new("hello");
    /// let result = state.finish(());
    /// assert_eq!(result.map_inner(|_| 1), ParseResult::Pending(state, 1));
    /// ```
    #[inline(always)]
    pub fn map_inner<F, U>(self, f: F) -> ParseResult<'i, U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Self::Pending(state, value) => ParseResult::Pending(state, f(value)),
            Self::Stop(reason) => ParseResult::Stop(reason),
        }
    }
    /// Dispatch branch events based on the result
    ///
    /// # Examples
    ///
    /// ```
    /// # use pex::{ParseResult, ParseState};
    /// let state = ParseState::new("hello");
    /// let result = state.finish(());
    /// result.dispatch(|ok| println!("ok: {:?}", ok), |fail| println!("fail: {:?}", fail));
    /// ```
    #[inline(always)]
    pub fn dispatch<F, G>(self, ok: F, fail: G) -> Self
    where
        F: FnOnce(ParseState),
        G: FnOnce(StopBecause),
    {
        match &self {
            ParseResult::Pending(data, _) => ok(*data),
            ParseResult::Stop(stop) => fail(*stop),
        }
        self
    }
    /// Convert a parse [`Result`](Self) to a std [`Result`]
    ///
    /// # Examples
    ///
    /// ```
    /// # use pex::{ParseResult, ParseState};
    /// let state = ParseState::new("hello");
    /// let result = state.finish(());
    /// assert_eq!(result.as_result(), Ok((state, ())));
    /// ```
    #[inline(always)]
    #[allow(clippy::wrong_self_convention)]
    pub fn as_result(self) -> Result<Parsed<'i, T>, StopBecause> {
        match self {
            Self::Pending(state, value) => Ok((state, value)),
            Self::Stop(reason) => Err(reason),
        }
    }
}
