use std::collections::HashSet;
use std::fmt;

use phf_generator::HashState;
use phf_shared::checksum;

/// A builder for the `phf::CompressedMap` type.
pub struct Map<'a> {
    datum_len: usize,
    keys: Vec<&'a str>,
    values: Vec<&'a str>,
    path: String,
}

impl<'a> Map<'a> {
    /// Creates a new `phf::CompressedMap` builder.
    pub fn new(datum_len: usize) -> Map<'a> {
        // FIXME rust#27438
        //
        // On Windows/MSVC there are major problems with the handling of dllimport.
        // Here, because downstream build scripts only invoke generics from phf_codegen,
        // the linker ends up throwing a way a bunch of static symbols we actually need.
        // This works around the problem, assuming that all clients call `Map::new` by
        // calling a non-generic function.
        fn noop_fix_for_27438() {}
        noop_fix_for_27438();

        Map {
            datum_len,
            keys: vec![],
            values: vec![],
            path: String::from("::phf"),
        }
    }

    /// Set the path to the `phf` crate from the global namespace
    pub fn phf_path(&mut self, path: &str) -> &mut Map<'a> {
        self.path = path.to_owned();
        self
    }

    /// Adds an entry to the builder.
    ///
    /// `value` will be written exactly as provided in the constructed source.
    pub fn entry(&mut self, key: &'a str, value: &'a str) -> &mut Map<'a> {
        assert_eq!(value.len(), self.datum_len, "{}, {}", key, value);
        self.keys.push(key);
        self.values.push(value);
        self
    }

    /// Calculate the hash parameters and return a struct implementing
    /// [`Display`](::std::fmt::Display) which will print the constructed
    /// `phf::CompressedMap`.
    ///
    /// # Panics
    ///
    /// Panics if there are any duplicate keys.
    pub fn build(&self, seed: u64) -> DisplayMap {
        let mut set = HashSet::new();
        for key in &self.keys {
            if !set.insert(key) {
                panic!("duplicate key `{}`", key);
            }
        }

        let state = phf_generator::generate_hash(self.keys.as_ref(), seed);

        DisplayMap {
            datum_len: self.datum_len,
            path: &self.path,
            keys: &self.keys,
            values: &self.values,
            state,
        }
    }
}

/// An adapter for printing a [`Map`](Map).
#[derive(Debug)]
pub struct DisplayMap<'a, 'b> {
    pub datum_len: usize,
    pub path: &'a str,
    pub state: HashState,
    pub keys: &'a [&'b str],
    pub values: &'a [&'b str],
}

impl<'a, 'b> DisplayMap<'a, 'b> {
    /// Return an entry from the map
    pub fn get(&self, key: &str) -> Option<&str> {
        if self.state.disps.len() == 0 {
            return None;
        }
        let hashes = phf_shared::hash(key, &self.state.key);
        let index = phf_shared::get_index(&hashes, &*self.state.disps, self.keys.len());
        let calc_key = self.keys[self.state.map[index as usize]];
        let value = self.values[self.state.map[index as usize]];
        if phf_shared::checksum(calc_key) == phf_shared::checksum(key) {
            Some(value.trim_end_matches('\0'))
        } else {
            None
        }
    }
}

impl<'a, 'b> fmt::Display for DisplayMap<'a, 'b> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // funky formatting here for nice output
        write!(
            f,
            "{}::Map {{
    datum_len: {},
    key: {:?},
    disps: {}::Slice::Static(&[",
            self.path, self.datum_len, self.state.key, self.path
        )?;

        // write map displacements
        for &(d1, d2) in &self.state.disps {
            write!(
                f,
                "
        ({}, {}),",
                d1, d2
            )?;
        }

        write!(
            f,
            "
    ]),
    keys: {}::Slice::Static(&[",
            self.path
        )?;

        // write map keys
        for &idx in &self.state.map {
            write!(
                f,
                "
        {},",
                checksum(&self.keys[idx]),
            )?;
        }

        write!(
            f,
            "
    ]),
    data: {}::Slice::Static(&[",
            self.path
        )?;

        // write map values
        for &idx in &self.state.map {
            write!(
                f,
                "
        // {}
        ",
                self.values[idx].trim_end_matches('\0')
            )?;
            for b in self.values[idx].as_bytes() {
                write!(f, "{:#04x?},", b)?;
            }
        }

        write!(
            f,
            "
    ]),
}}"
        )
    }
}
