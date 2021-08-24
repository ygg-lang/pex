use crate::third_party::const_hasher::fnv1a_hash_str_64;

/// Interned string
pub struct InternString {
    key: u64,
}

impl InternString {
    pub const fn new_static(string: &'static str) -> Self {
        Self { key: fnv1a_hash_str_64(string) }
    }
    pub fn new_runtime(string: &str) -> Self {
        Self { key: fnv1a_hash_str_64(string) }
    }
}
