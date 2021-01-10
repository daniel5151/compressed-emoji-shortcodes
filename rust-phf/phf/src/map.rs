//! An immutable map constructed at compile time.
use crate::Slice;
use phf_shared::{self, checksum, HashKey};

/// An immutable map constructed at compile time.
///
/// ## Note
///
/// The fields of this struct are public so that they may be initialized by the
/// `phf_map!` macro and code generation. They are subject to change at any
/// time and should never be accessed directly.
pub struct Map<V: 'static> {
    #[doc(hidden)]
    pub key: HashKey,
    #[doc(hidden)]
    pub disps: Slice<(u32, u32)>,
    #[doc(hidden)]
    pub entries: Slice<(u8, V)>,
}

impl<V> Map<V> {
    /// Returns true if the `Map` is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of entries in the `Map`.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Like `get`, but returns both the key and the value.
    pub fn get(&self, key: &str) -> Option<&V> {
        if self.disps.len() == 0 {
            return None;
        } //Prevent panic on empty map
        let hashes = phf_shared::hash(key, &self.key);
        let index = phf_shared::get_index(&hashes, &*self.disps, self.entries.len());
        let entry = &self.entries[index as usize];
        if entry.0 == checksum(key) {
            Some(&entry.1)
        } else {
            None
        }
    }
}
