




```rust
let xid = UnicodeSet::new("xid_start_trie").with_ranges(XID_START);
println!("{:?}", xid);
println!("{}", xid.export_rust_code().unwrap());
```