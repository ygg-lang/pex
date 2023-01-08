use super::*;

impl<'i, T> Debug for ParseResult<'i, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            ParseResult::Pending(s, v) => f
                .debug_struct("Pending")
                .field("value", v)
                .field("rest_text", &s.residual)
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
    /// Map inner value into target
    ///
    /// ```
    /// # use pex::{ParseResult, ParseState};
    /// let state = ParseState::new("hello");
    /// let result = state.finish(());
    /// assert_eq!(result.map_inner(|_| 1), ParseResult::Pending(state, 1));
    /// ```
    #[inline(always)]
    pub fn map_into<U>(self) -> ParseResult<'i, U>
    where
        T: Into<U>,
    {
        self.map_inner(Into::into)
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
    /// Returns the contained [`ParseResult::Pending`] value, drop current state, panic if state reach stopped.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// # use pex::{ParseResult, ParseState};
    /// let state = ParseState::new("hello");
    /// let result = state.finish(());
    /// assert_eq!(result.as_result(), Ok((state, ())));
    /// ```
    #[track_caller]
    #[inline(always)]
    pub fn unwrap(self) -> T {
        match self {
            ParseResult::Pending(_, v) => v,
            ParseResult::Stop(e) => panic!("{e:?}"),
        }
    }
    /// Check whether a match is successful, note that an empty match is always successful.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pex::{ParseResult, ParseState};
    /// # use pex::helpers::{decimal_string, quotation_pair};
    /// let state = ParseState::new("'hello'");
    /// assert!(state.match_fn(|s| quotation_pair(s, '"', '"')).is_failure());
    /// assert!(state.match_fn(decimal_string).is_failure());
    /// ```
    #[inline(always)]
    pub fn is_success(&self) -> bool {
        match self {
            Self::Pending(..) => true,
            Self::Stop(..) => false,
        }
    }
    /// Check whether a match is failed, note that an empty match never fails.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pex::{ParseResult, ParseState};
    /// # use pex::helpers::{decimal_string, quotation_pair};
    /// let state = ParseState::new("'hello'");
    /// assert!(!state.match_fn(|s| quotation_pair(s, '\'', '\'')).is_failure());
    /// assert!(state.match_fn(decimal_string).is_failure());
    /// ```
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        match self {
            Self::Pending(..) => false,
            Self::Stop(..) => true,
        }
    }
}
