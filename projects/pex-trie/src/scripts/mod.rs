use crate::generate::xid::XID_START;
use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};
use ucd_trie::{Error, TrieSetOwned, TrieSetSlice};

pub fn build_slice(ranges: &'static [(char, char)]) -> ucd_trie::Result<TrieSetSlice<'static>> {
    unsafe {
        let slice = owned.as_slice();
        // elide the lifetime
        Ok(std::mem::transmute(slice))
    }
}

pub struct TrieSet {
    max_width: usize,
    trie: TrieSetOwned,
}

impl Display for TrieSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TrieSet {
    pub fn from_ranges(ranges: &[(char, char)]) -> Result<Self, Error> {
        let mut set = BTreeSet::new();
        for (start, end) in ranges {
            for i in *start..=*end {
                set.insert(i);
            }
        }
        let owned = TrieSetOwned::from_scalars(set)?;
        Ok(Self { max_width: 144, trie: owned })
    }
}

#[test]
fn test() {
    println!("{:?}", build_slice(XID_START));
}
