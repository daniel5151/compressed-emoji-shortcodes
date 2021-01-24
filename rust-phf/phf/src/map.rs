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
pub struct Map {
    #[doc(hidden)]
    pub key: HashKey,
    #[doc(hidden)]
    pub disps: Slice<(u32, u32)>,
    #[doc(hidden)]
    pub keys: Slice<u16>,
    #[doc(hidden)]
    pub data: Slice<u8>,
    #[doc(hidden)]
    pub datum_len: u8,
}

impl Map {
    /// Returns true if the `Map` is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of entries in the `Map`.
    pub fn len(&self) -> usize {
        self.keys.len()
    }

    /// Return an entry from the map
    pub fn get(&self, key: &str) -> Option<&str> {
        if self.disps.len() == 0 {
            return None;
        }
        let hashes = phf_shared::hash(key, &self.key);
        let index = phf_shared::get_index(&hashes, &*self.disps, self.keys.len());
        let calc_key = self.keys[index as usize];
        let value =
            &self.data[(index as usize * self.datum_len as usize)..][..self.datum_len as usize];
        if calc_key == checksum(key) {
            Some(unsafe { core::str::from_utf8_unchecked(value).trim_end_matches('\0') })
        } else {
            None
        }
    }
}
