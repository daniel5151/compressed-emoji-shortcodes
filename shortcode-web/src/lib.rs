#![no_std]

mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Char {
    pub found: bool,
    pub first: Option<char>,
    pub second: Option<char>,
    pub third: Option<char>,
}

#[wasm_bindgen]
pub fn shortcode(s: &str) -> Char {
    let mut ret = Char {
        found: false,
        first: None,
        second: None,
        third: None,
    };

    match maximally_compressed_gh_shortcodes::lookup(s) {
        None => ret,
        Some(s) => {
            ret.found = true;
            for (i, c) in s.chars().enumerate() {
                match i {
                    0 => ret.first = Some(c),
                    1 => ret.second = Some(c),
                    2 => ret.third = Some(c),
                    _ => unreachable!(),
                }
            }
            ret
        }
    }
}
