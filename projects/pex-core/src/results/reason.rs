use super::*;

impl Default for StopBecause {
    fn default() -> Self {
        Self::Uninitialized
    }
}

impl Error for StopBecause {}

impl Display for StopBecause {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            StopBecause::Uninitialized => f.write_str("Uninitialized"),
            StopBecause::ExpectEOF { .. } => f.write_str("Expect end of file"),
            StopBecause::ExpectRepeats { min, current, .. } => {
                f.write_fmt(format_args!("Expect at least {} repeats (got {})", min, current))
            }
            StopBecause::MissingCharacterSet { expected, .. } => {
                f.write_fmt(format_args!("Missing character set `{}`", expected))
            }
            StopBecause::MissingCharacterRange { start, end, .. } => match start.eq(end) {
                true => f.write_fmt(format_args!("Missing character '{}'", start)),
                false => f.write_fmt(format_args!("Expect character in range '{}'..='{}'", start, end)),
            },
            StopBecause::MissingString { message, .. } => f.write_fmt(format_args!("Missing string '{}'", message)),
            StopBecause::MustBe { message, .. } => f.write_fmt(format_args!("Must be `{}`", message)),
            StopBecause::ShouldNotBe { message, .. } => f.write_fmt(format_args!("Should not be `{}`", message)),
            StopBecause::Custom(v) => f.write_fmt(format_args!("Custom error: {}", v)),
        }
    }
}

impl<'i> Error for CustomError<'i> {}

impl<'i> Debug for CustomError<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CustomError")
            .field("message", &self.message)
            .field("range", &Range { start: self.start, end: self.end })
            .finish()
    }
}

impl<'i> Display for CustomError<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.message)
    }
}

impl From<CustomError<'static>> for StopBecause {
    fn from(value: CustomError<'static>) -> Self {
        Self::Custom(value)
    }
}

impl StopBecause {
    /// Create a new `StopBecause::MustBe` error
    pub const fn must_be<T>(message: &'static str, position: usize) -> Result<T, StopBecause> {
        Err(Self::MustBe { message, position })
    }
    /// Create a new `StopBecause::ExpectEof` error
    pub const fn expect_eof<T>(position: usize) -> Result<T, StopBecause> {
        Err(Self::ExpectEOF { position })
    }
    /// Create a new `StopBecause::MissingCharacter` error
    pub const fn missing_character<T>(expected: char, position: usize) -> Result<T, StopBecause> {
        Err(Self::MissingCharacterRange { start: expected, end: expected, position })
    }
    /// Create a new `StopBecause::MissingCharacterRange` error
    pub const fn missing_character_range<T>(start: char, end: char, position: usize) -> Result<T, StopBecause> {
        Err(Self::MissingCharacterRange { start, end, position })
    }
    /// Create a new `StopBecause::MissingCharacter` error
    pub const fn missing_character_set<T>(expected: &'static str, position: usize) -> Result<T, StopBecause> {
        Err(Self::MissingCharacterSet { expected, position })
    }
    /// Create a new `StopBecause::MissingString` error
    pub const fn missing_string<T>(message: &'static str, position: usize) -> Result<T, StopBecause> {
        Err(Self::MissingString { message, position })
    }
    /// Create a new [CustomError]
    pub const fn custom_error<T>(message: &'static str, start: usize, end: usize) -> Result<T, StopBecause> {
        Err(Self::Custom(CustomError { message, start, end }))
    }
    /// Create a new `StopBecause::Custom` error
    pub const fn range(&self) -> Range<usize> {
        match *self {
            StopBecause::Uninitialized => 0..0,
            StopBecause::ExpectEOF { position } => position..position + 1,
            StopBecause::ExpectRepeats { min: _, current: _, position } => position..position + 1,
            StopBecause::MissingCharacterSet { expected: _, position } => position..position + 1,
            StopBecause::MissingCharacterRange { start: _, end: _, position } => position..position + 1,
            StopBecause::MissingString { message, position } => position..position + message.len(),
            StopBecause::MustBe { message: _, position } => position..position + 1,
            StopBecause::ShouldNotBe { message: _, position } => position..position + 1,
            StopBecause::Custom(e) => e.range(),
        }
    }
}

impl<'i> CustomError<'i> {
    /// Create a new [CustomError]
    pub const fn range(&self) -> Range<usize> {
        self.start..self.end
    }
}
