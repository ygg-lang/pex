use crate::string::ShallowString;
use std::{borrow::Cow, fmt::Debug};

/// A builder to create the `ShallowString`.
///
/// # Examples
///
/// ```
/// # use shallow::CharacterShallow;
/// const TEXT10: &str = "1234567890";
/// const TEXT21: &str = "1234567890_1234567890";
/// const TEXT27: &str = "1234567890_1234567890_12345";
/// let sb = CharacterShallow::new(21, 5);
/// assert_eq!(sb.build_cow(TEXT10), TEXT10); // not change
/// assert_eq!(sb.build_cow(TEXT21), "1234567890_1234567890"); // not change
/// assert_eq!(sb.build_cow(TEXT27), "123456789 <...> 12345"); // shorten
/// let sb = sb.with_shallow_text("...");
/// assert_eq!(sb.build_cow(TEXT27), "1234567890_12...12345"); // shorten
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct CharacterShallow {
    /// The max width of the [ShallowString]
    pub max_width: usize,
    /// The reserved end width of the [ShallowString]
    pub end_reserved: usize,
    /// The shallow text if
    pub shallow_text: String,
}

/// Word level shallow
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct WordShallow {}

/// Defines how to calculate string length
pub enum CounterMode {
    ///
    Bytes,
    ///
    Characters,
}

/// Change the shallow mode
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ShallowMode {
    /// Add a place holder
    PlaceHolder {
        ///
        text: Cow<'static, str>,
    },
    /// Add a text counter
    Counter {
        /// text before number
        lhs: Cow<'static, str>,
        /// text after number
        rhs: Cow<'static, str>,
    },
}

impl CounterMode {
    ///
    pub fn count(&self, raw: &str) -> usize {
        match self {
            CounterMode::Bytes => raw.len(),
            CounterMode::Characters => raw.chars().count(),
        }
    }
}

impl ShallowMode {
    /// Get the shallow text width
    pub fn size_hint(&self, counter: CounterMode, fill: &str) -> usize {
        match self {
            ShallowMode::PlaceHolder { text } => match counter {
                CounterMode::Bytes => text.len(),
                CounterMode::Characters => text.chars().count(),
            },
            ShallowMode::Counter { lhs, rhs } => match counter {
                CounterMode::Bytes => {
                    let start = lhs.len();
                    let middle = fill.len();
                    let end = rhs.len();
                    start + middle + end
                }
                CounterMode::Characters => {
                    let start = lhs.chars().count();
                    let middle = fill.chars().count();
                    let end = rhs.chars().count();
                    start + middle + end
                }
            },
        }
    }
}

impl Default for CharacterShallow {
    fn default() -> Self {
        Self { max_width: 144, end_reserved: 42, shallow_text: " <...> ".to_string() }
    }
}

impl CharacterShallow {
    /// build a ss
    pub fn new(width: usize, reversed: usize) -> Self {
        Self { max_width: width, end_reserved: reversed, ..Self::default() }
    }
    /// build a ss
    pub fn with_width(mut self, width: usize) -> Self {
        self.max_width = width;
        self
    }
    /// build a ss
    pub fn set_delta_width(&mut self, delta: isize) {
        let new = if delta >= 0 {
            self.max_width.saturating_add(delta as usize)
        }
        else {
            self.max_width.saturating_sub(delta.abs() as usize)
        };
        self.max_width = new;
    }
    /// build a ss
    pub fn with_delta_width(mut self, delta: isize) -> Self {
        self.set_delta_width(delta);
        self
    }
    /// build a ss
    pub fn with_shallow_text(mut self, shallow_text: &str) -> Self {
        self.shallow_text = shallow_text.to_string();
        self
    }
    /// build a ss
    pub fn with_end_reserved(mut self, end_reserved: usize) -> Self {
        self.end_reserved = end_reserved;
        self
    }
    /// build a ss
    pub fn build<'s>(&self, raw: &'s str) -> ShallowString<'s> {
        if raw.len() <= self.max_width {
            return ShallowString { raw: Cow::Borrowed(raw) };
        }
        let mut string = String::with_capacity(self.max_width);
        let start_len = self.max_width - self.end_reserved - self.shallow_text.len();
        // SAFETY: slice is always less than raw.len()
        unsafe {
            let start = raw.get_unchecked(..start_len);
            let middle = self.shallow_text.as_str();
            let end = raw.get_unchecked(raw.len() - self.end_reserved..);
            string.push_str(start);
            string.push_str(middle);
            string.push_str(end);
        }
        ShallowString { raw: Cow::Owned(string) }
    }
    /// Build a cow
    pub fn build_cow<'s>(&self, raw: &'s str) -> Cow<'s, str> {
        self.build(raw).raw
    }
}
