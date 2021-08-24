use super::*;

impl Default for StopBecause {
    fn default() -> Self {
        Self::Uninitialized
    }
}

impl Error for StopBecause {}

impl Display for StopBecause {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StopBecause::Uninitialized => f.write_str("Uninitialized"),
            StopBecause::ExpectEof { .. } => f.write_str("Expect end of file"),
            StopBecause::ExpectRepeats { min, current, .. } => {
                f.write_fmt(format_args!("Expect at least {} repeats (got {})", min, current))
            }
            StopBecause::MissingCharacter { expected, .. } => f.write_fmt(format_args!("Missing character '{}'", expected)),
            StopBecause::MissingCharacterRange { start, end, .. } => {
                f.write_fmt(format_args!("Expect character in range '{}'..='{}'", start, end))
            }
            StopBecause::MissingString { message, .. } => f.write_fmt(format_args!("Missing string '{}'", message)),
            StopBecause::MustBe { message, .. } => f.write_fmt(format_args!("Must be `{}`", message)),
            StopBecause::ShouldNotBe { message, .. } => f.write_fmt(format_args!("Should not be `{}`", message)),
            StopBecause::Custom(v) => f.write_fmt(format_args!("Custom error: {}", v)),
        }
    }
}

impl Error for CustomError {}

impl Debug for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CustomError")
            .field("message", &self.message)
            .field("range", &Range { start: self.start, end: self.end })
            .finish()
    }
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl StopBecause {
    /// Create a new `StopBecause::MustBe` error
    pub const fn must_be<T>(message: &'static str, position: usize) -> Result<T, StopBecause> {
        Err(Self::MustBe { message, position })
    }
    /// Create a new `StopBecause::ExpectEof` error
    pub const fn expect_eof<T>(position: usize) -> Result<T, StopBecause> {
        Err(Self::ExpectEof { position })
    }
    /// Create a new `StopBecause::MissingCharacter` error
    pub const fn missing_character<T>(expected: char, position: usize) -> Result<T, StopBecause> {
        Err(Self::MissingCharacter { expected, position })
    }
    /// Create a new `StopBecause::MissingCharacterRange` error
    pub const fn missing_character_range<T>(start: char, end: char, position: usize) -> Result<T, StopBecause> {
        Err(Self::MissingCharacterRange { start, end, position })
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
            StopBecause::ExpectEof { position } => position..position + 1,
            StopBecause::ExpectRepeats { min: _, current: _, position } => position..position + 1,
            StopBecause::MissingCharacter { expected, position } => position..position + expected.len_utf8(),
            StopBecause::MissingCharacterRange { start: _, end: _, position } => position..position + 1,
            StopBecause::MissingString { message, position } => position..position + message.len(),
            StopBecause::MustBe { message: _, position } => position..position + 1,
            StopBecause::ShouldNotBe { message: _, position } => position..position + 1,
            StopBecause::Custom(CustomError { start, end, .. }) => start..end,
        }
    }
}
