use super::*;

/// Character parsing methods.
impl<'i> ParseState<'i> {
    /// Match a single character.
    ///
    /// ```ygg
    /// 'c'
    /// ```
    #[inline]
    pub fn match_char(self, target: char) -> ParseResult<'i, char> {
        match self.get_character(0) {
            Some(c) if c.eq(&target) => self.advance(target).finish(target),
            _ => StopBecause::missing_character(target, self.start_offset)?,
        }
    }
    /// Match a character in given range.
    ///
    /// ```ygg
    /// [a-z]
    /// ```
    #[inline]
    pub fn match_char_range(self, start: char, end: char) -> ParseResult<'i, char> {
        match self.get_character(0) {
            Some(c) if c <= end && c >= start => self.advance(c).finish(c),
            _ => StopBecause::missing_character_range(start, end, self.start_offset)?,
        }
    }
    /// Assert end of file
    /// ```ygg
    /// p $
    /// ```
    #[inline]
    pub fn match_eof(self) -> ParseResult<'i, ()> {
        match self.get_character(0) {
            Some(_) => StopBecause::expect_eof(self.start_offset)?,
            None => self.finish(()),
        }
    }
    /// Match any character, except `EOF`.
    #[inline]
    pub fn match_char_any(self) -> ParseResult<'i, char> {
        self.match_char_if(|_| true, "ANY")
    }
    /// Match a character with given set.
    #[inline]
    #[cfg(feature = "ucd-trie")]
    pub fn match_char_set(self, set: TrieSetSlice, message: &'static str) -> ParseResult<'i, char> {
        self.match_char_if(|c| set.contains_char(c), message)
    }
    /// Parsing a character with given rule.
    #[inline]
    pub fn match_char_if<F>(self, mut predicate: F, message: &'static str) -> ParseResult<'i, char>
    where
        F: FnMut(char) -> bool,
    {
        match self.get_character(0) {
            Some(c) if predicate(c) => self.advance(c).finish(c),
            _ => StopBecause::must_be(message, self.start_offset)?,
        }
    }
}

impl<'i> ParseState<'i> {
    /// Match a static string.
    #[inline]
    pub fn match_str(self, target: &'static str) -> ParseResult<'i, &'i str> {
        let s = match self.get_string(0..target.len()) {
            Some(s) if s.eq(target) => s.len(),
            _ => StopBecause::missing_string(target, self.start_offset)?,
        };
        self.advance_view(s)
    }
    /// Match a static string.
    #[inline]
    pub fn match_str_insensitive(self, target: &'static str) -> ParseResult<'i, &'i str> {
        let s = match self.get_string(0..target.len()) {
            Some(s) if s.eq_ignore_ascii_case(target) => s.len(),
            _ => StopBecause::missing_string(target, self.start_offset)?,
        };
        self.advance_view(s)
    }
    /// Match a string with given regex.
    #[cfg(feature = "regex-automata")]
    pub fn match_regex(&self, re: &Regex, message: &'static str) -> ParseResult<'i, MultiMatch> {
        match re.try_find_leftmost(self.rest_text.as_bytes()) {
            Ok(Some(m)) => {
                let new = self.advance(m.end());
                Pending(new, m)
            }
            Ok(None) => StopBecause::must_be(message, self.start_offset)?,
            Err(e) => {
                eprintln!("Error: {:?}", e);
                unimplemented!()
            }
        }
    }

    /// Match a string with given conditional.
    #[inline]
    pub fn match_str_if<F>(self, mut predicate: F, message: &'static str) -> ParseResult<'i, &'i str>
    where
        F: FnMut(char) -> bool,
    {
        let mut offset = 0;
        for char in self.rest_text.chars() {
            match predicate(char) {
                true => offset += char.len_utf8(),
                false => break,
            }
        }
        if offset == 0 {
            StopBecause::missing_string(message, self.start_offset)?;
        }
        self.advance(offset).finish(&self.rest_text[..offset])
    }
}

impl<'i> ParseState<'i> {
    /// Simple suffix call form
    #[inline]
    pub fn match_fn<T, F>(self, mut parse: F) -> ParseResult<'i, T>
    where
        F: FnMut(ParseState<'i>) -> ParseResult<T>,
    {
        parse(self)
    }
    /// Parses a sequence of 0 or more repetitions of the given parser.
    /// ```regex
    /// p*
    /// p+ <=> p p*
    /// ```
    #[inline]
    pub fn match_repeats<T, F>(self, mut parse: F) -> ParseResult<'i, Vec<T>>
    where
        F: FnMut(ParseState<'i>) -> ParseResult<T>,
    {
        let mut result = Vec::new();
        let mut state = self;
        loop {
            match parse(state) {
                Pending(new, value) => {
                    state = new;
                    result.push(value);
                }
                Stop(_) => break,
            }
        }
        state.finish(result)
    }

    /// Parses a sequence of 0 or more repetitions of the given parser.
    /// ```regex
    /// p* <=> p{0, \inf}
    /// p+ <=> p{1, \inf}
    /// p{min, max}
    /// ```
    #[inline]
    pub fn match_repeat_m_n<T, F>(self, min: usize, max: usize, mut parse: F) -> ParseResult<'i, Vec<T>>
    where
        F: FnMut(ParseState<'i>) -> ParseResult<T>,
    {
        let mut result = Vec::new();
        let mut count = 0;
        let position = self.start_offset;
        let mut state = self;
        loop {
            match parse(state.clone()) {
                Pending(new, value) => {
                    state = new;
                    result.push(value);
                    count += 1;
                    if count >= max {
                        break;
                    }
                }
                Stop(_) => break,
            };
        }
        if count < min {
            Err(StopBecause::ExpectRepeats { min, current: count, position })?
        }
        state.finish(result)
    }
    /// Parse an optional element
    /// ```regex
    /// p?
    /// ```
    #[inline]
    pub fn match_optional<T, F>(self, mut parse: F) -> ParseResult<'i, Option<T>>
    where
        F: FnMut(ParseState<'i>) -> ParseResult<T>,
    {
        match parse(self.clone()) {
            Pending(state, value) => state.finish(Some(value)),
            Stop(_) => self.finish(None),
        }
    }
    /// Match but does not return the result
    #[inline]
    pub fn skip<F, T>(self, mut parse: F) -> ParseState<'i>
    where
        F: FnMut(ParseState<'i>) -> ParseResult<T>,
    {
        match parse(self.clone()) {
            Pending(new, _) => new,
            Stop(_) => self,
        }
    }
    /// Zero-width positive match, does not consume input
    ///
    /// Used to be a external rule, which used as assert
    ///
    /// ```regex
    /// &ahead p
    /// p &after
    /// ```
    #[inline]
    pub fn match_positive<F, T>(self, mut parse: F, message: &'static str) -> ParseResult<'i, ()>
    where
        F: FnMut(ParseState<'i>) -> ParseResult<T>,
    {
        match parse(self.clone()) {
            Pending(..) => self.finish(()),
            Stop(_) => Stop(StopBecause::MustBe { message, position: self.start_offset }),
        }
    }
    /// Zero-width negative match, does not consume input
    /// ```regex
    /// !ahead p
    /// p !after
    /// ```
    #[inline]
    pub fn match_negative<F, T>(self, mut parse: F, message: &'static str) -> ParseResult<'i, ()>
    where
        F: FnMut(ParseState<'i>) -> ParseResult<T>,
    {
        match parse(self.clone()) {
            Pending(..) => Stop(StopBecause::ShouldNotBe { message, position: self.start_offset }),
            Stop(_) => self.finish(()),
        }
    }
}

impl<'i> ParseState<'i> {
    /// Parse a comment line
    /// ```regex
    /// # comment
    /// // comment
    /// ```
    #[inline]
    pub fn match_comment_line(self, head: &'static str) -> ParseResult<'i, &'i str> {
        if !self.rest_text.starts_with(head) {
            StopBecause::missing_string(head, self.start_offset)?;
        }
        let offset = match self.rest_text.find(|c| c == '\r' || c == '\n') {
            Some(s) => s,
            None => self.rest_text.len(),
        };
        self.advance(offset).finish(&self.rest_text[..offset])
    }
    /// Parse the comment block
    ///
    /// ```ygg
    /// /* */
    /// ```
    #[inline]
    pub fn match_comment_block<F, T>(self, head: &'static str, tail: &'static str) -> ParseResult<'i, ()>
    where
        F: FnMut(ParseState<'i>) -> ParseResult<T>,
    {
        if !self.rest_text.starts_with(head) {
            Stop::<()>(StopBecause::MissingString { message: head, position: self.start_offset })?;
        }
        let mut offset = head.len();
        let rest = &self.rest_text[offset..];
        match rest.find(tail) {
            Some(s) => offset += s + tail.len(),
            None => StopBecause::missing_string(tail, self.start_offset + offset)?,
        }
        self.advance(offset).finish(())
    }
    /// Parse the comment block
    ///
    /// ```ygg
    /// r#" "#
    /// r##" "##
    /// r###" "###
    /// ```
    pub fn match_surround<F, T>(self, delimiter: char, min: usize) -> ParseResult<'i, ()>
    where
        F: FnMut(ParseState<'i>) -> ParseResult<T>,
    {
        let mut count = 0;
        for c in self.rest_text.chars() {
            match c == delimiter {
                true => count += 1,
                false => break,
            }
        }
        if count == 0 {
            StopBecause::missing_string("r#", self.start_offset)?
        }
        if count < min {
            StopBecause::missing_string("r##", self.start_offset)?
        }
        let head = count * delimiter.len_utf8();
        let rest = &self.rest_text[head..];
        let end = delimiter.to_string().repeat(count);
        match rest.find(&end) {
            Some(s) => self.advance(s + count * delimiter.len_utf8()).finish(()),
            None => StopBecause::missing_string("match_raw_paired", self.start_offset + count)?,
        }
    }
}
