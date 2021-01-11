#![no_std]

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn lookup(s: &str) -> u64 {
    *maximally_compressed_gh_shortcodes::EMOJI
        .get(s)
        .unwrap_or(&0)
}
