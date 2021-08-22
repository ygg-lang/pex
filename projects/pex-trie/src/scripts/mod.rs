use crate::generate::xid::XID_START;
use ucd_trie::TrieSetOwned;

fn xid() -> ucd_trie::Result<TrieSetOwned> {
    TrieSetOwned::from_codepoints(XID_START)
}

#[test]
fn test() {
    println!("{:?}", xid());
}
