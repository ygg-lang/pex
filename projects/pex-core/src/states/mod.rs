use std::{ops::Range, slice::SliceIndex};

#[cfg(feature = "regex-automata")]
use regex_automata::{dfa::regex::Regex, MultiMatch};
#[cfg(feature = "ucd-trie")]
use ucd_trie::TrieSetSlice;

use crate::{
    results::StopBecause,
    ParseResult,
    ParseResult::{Pending, Stop},
};

pub mod advance;
mod builtin;
pub mod choice;
mod concat;

/// Represent a parsed value
pub type Parsed<'i, T> = (ParseState<'i>, T);

/// The state of parsing
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ParseState<'i> {
    /// Rest part of string
    pub rest_text: &'i str,
    /// Start offset of the string
    pub start_offset: usize,
    /// Stop reason
    pub stop_reason: Option<StopBecause>,
}

impl<'i> ParseState<'i> {
    /// Create a new state
    #[inline(always)]
    pub fn new(input: &'i str) -> Self {
        Self { rest_text: input, start_offset: 0, stop_reason: None }
    }
    /// Reset the cursor offset
    #[inline(always)]
    pub fn with_start_offset(mut self, offset: usize) -> Self {
        self.start_offset = offset;
        self
    }
    /// Finish with given value
    #[inline(always)]
    pub fn finish<T>(self, value: T) -> ParseResult<'i, T> {
        Pending(self, value)
    }
    /// Check if the string is depleted
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.rest_text.is_empty()
    }
    /// Get inner error
    #[inline(always)]
    pub fn get_error(self) -> StopBecause {
        match self.stop_reason {
            Some(s) => s,
            None => StopBecause::Uninitialized,
        }
    }
    /// Set inner error
    #[inline(always)]
    pub fn set_error(&mut self, error: StopBecause) {
        self.stop_reason = Some(error);
    }
    /// Get a string view
    #[inline(always)]
    pub fn get_string<R>(&self, range: R) -> Option<&R::Output>
    where
        R: SliceIndex<str>,
    {
        self.rest_text.get(range)
    }
    /// Get nth character
    #[inline(always)]
    pub fn get_character(&self, nth: usize) -> Option<char> {
        self.rest_text.chars().nth(nth)
    }
    /// Get range away from start state
    #[inline(always)]
    pub fn away_from(&self, start: ParseState) -> Range<usize> {
        start.start_offset..self.start_offset
    }
}
