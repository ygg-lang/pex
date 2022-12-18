use crate::writer::EasyWrite;
use std::{
    collections::BTreeSet,
    fmt::{Debug, Display, Formatter, Write},
};
use ucd_trie::TrieSetOwned;

/// A unicode character set
pub struct UnicodeSet {
    name: String,
    max_width: usize,
    set: BTreeSet<char>,
}

impl Debug for UnicodeSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnicodeSet").field("name", &self.name).field("count", &self.set.len()).finish()
    }
}

impl UnicodeSet {
    /// Create a new set with the given name
    pub fn new(set: &str) -> Self {
        let mut name = String::with_capacity(set.len());
        for c in set.chars() {
            if c.is_ascii_alphanumeric() {
                // upper
                name.push(c.to_ascii_uppercase());
            }
            else {
                name.push('_');
            }
        }

        Self { name, max_width: 144, set: BTreeSet::new() }
    }
    /// Add a range of characters to the set
    pub fn with_ranges(mut self, ranges: &[(char, char)]) -> Self {
        for (start, end) in ranges {
            for i in *start..=*end {
                self.set.insert(i);
            }
        }
        self
    }
    /// Add a single character to the set
    pub fn with_chars<I>(mut self, chars: I) -> Self
    where
        I: IntoIterator<Item = char>,
    {
        for c in chars {
            self.set.insert(c);
        }
        self
    }
    /// Set the maximum width of a line
    pub fn with_max_width(mut self, max_width: usize) -> Self {
        assert!(max_width >= 42, "max_width must be at least 42");
        self.max_width = max_width;
        self
    }
    /// Export the set as a rust code
    pub fn export_rust_code(&self) -> Result<String, core::fmt::Error> {
        let name = self.name.as_str();
        let mut code = format!("#[rustfmt::skip]\nconst {name}: TrieSetSlice<'static> = TrieSetSlice");
        code.push_str(" {\n");
        let trie = TrieSetOwned::from_scalars(self.set.iter()).map_err(|_| core::fmt::Error)?;
        let trie = trie.as_slice();
        self.write_slice_numbers(&mut code, trie.tree1_level1, "tree1_level1")?;
        self.write_slice_numbers(&mut code, trie.tree2_level1, "tree2_level1")?;
        self.write_slice_numbers(&mut code, trie.tree2_level2, "tree2_level2")?;
        self.write_slice_numbers(&mut code, trie.tree3_level1, "tree3_level1")?;
        self.write_slice_numbers(&mut code, trie.tree3_level2, "tree3_level2")?;
        self.write_slice_numbers(&mut code, trie.tree3_level3, "tree3_level3")?;
        code.push_str("};\n");
        Ok(code)
    }
    fn write_slice_numbers<T: Display>(&self, buffer: &mut String, slice: &[T], field: &str) -> core::fmt::Result {
        buffer.write_indent(4)?;
        buffer.write_str(field)?;
        buffer.write_str(": ")?;
        buffer.write_slice_numbers(slice, self.max_width, 8)
    }
}
