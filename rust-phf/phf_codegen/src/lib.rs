use std::collections::HashSet;
use std::fmt;

use phf_generator::HashState;
use phf_shared::checksum;

/// A builder for the `phf::Map` type.
pub struct Map<'a> {
    keys: Vec<&'a str>,
    values: Vec<u64>,
    path: String,
}

impl<'a> Default for Map<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Map<'a> {
    /// Creates a new `phf::Map` builder.
    pub fn new() -> Map<'a> {
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
    pub fn entry(&mut self, key: &'a str, value: u64) -> &mut Map<'a> {
        self.keys.push(key);
        self.values.push(value);
        self
    }

    /// Calculate the hash parameters and return a struct implementing
    /// [`Display`](::std::fmt::Display) which will print the constructed
    /// `phf::Map`.
    ///
    /// # Panics
    ///
    /// Panics if there are any duplicate keys.
    pub fn build(&self) -> DisplayMap {
        let mut set = HashSet::new();
        for key in &self.keys {
            if !set.insert(key) {
                panic!("duplicate key `{}`", key);
            }
        }

        let state = phf_generator::generate_hash(self.keys.as_ref());

        DisplayMap {
            path: &self.path,
            keys: &self.keys,
            values: &self.values,
            state,
        }
    }
}

/// An adapter for printing a [`Map`](Map).
pub struct DisplayMap<'a, 'b> {
    path: &'a str,
    state: HashState,
    keys: &'a [&'b str],
    values: &'a [u64],
}

impl<'a, 'b> fmt::Display for DisplayMap<'a, 'b> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // funky formatting here for nice output
        write!(
            f,
            "{}::Map {{
    key: {:?},
    disps: {}::Slice::Static(&[",
            self.path, self.state.key, self.path
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
    values: {}::Slice::Static(&[",
            self.path
        )?;

        // write map values
        for &idx in &self.state.map {
            write!(
                f,
                "
        {:#x?},",
                &self.values[idx]
            )?;
        }

        write!(
            f,
            "
    ]),
}}"
        )
    }
}
