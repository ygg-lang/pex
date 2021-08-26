use std::{
    borrow::Cow,
    fmt::{Debug, Formatter},
};

/// A placeholder string type
pub struct ShallowString<'a> {
    pub(crate) raw: Cow<'a, str>,
}

impl<'a> Debug for ShallowString<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.raw.as_ref())
    }
}

impl<'a> AsRef<str> for ShallowString<'a> {
    fn as_ref(&self) -> &str {
        self.raw.as_ref()
    }
}
