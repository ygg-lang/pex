use crate::generate::xid::XID_START;
use std::{
    collections::BTreeSet,
    fmt::{Debug, Formatter},
};
use ucd_trie::{Error, TrieSetOwned, TrieSetSlice};

pub struct UnicodeSet {
    name: String,
    count: usize,
    max_width: usize,
    trie: TrieSetOwned,
}

impl Debug for UnicodeSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnicodeSet").field("name", &self.name).field("count", &self.count).finish()
    }
}

impl UnicodeSet {
    pub fn from_ranges(ranges: &[(char, char)]) -> Result<Self, Error> {
        let mut set = BTreeSet::new();
        for (start, end) in ranges {
            for i in *start..=*end {
                set.insert(i);
            }
        }
        let count = set.len();
        let owned = TrieSetOwned::from_scalars(set)?;
        Ok(Self { name: "UNICODE_SET".to_string(), max_width: 144, count, trie: owned })
    }
    pub fn export_rust_code(&self) -> String {
        let mut code = String::new();
        code.push_str("pub static UNICODE_SET: TrieSetSlice<'static> = TrieSetSlice {\n");
        code
    }
}

pub const UNICODE_SET: TrieSetSlice<'static> = TrieSetSlice {
    tree1_level1: &[],
    tree2_level1: &[],
    tree2_level2: &[],
    tree3_level1: &[],
    tree3_level2: &[],
    tree3_level3: &[],
};

#[test]
fn test() {
    println!("{:?}", UnicodeSet::from_ranges(XID_START));
}
