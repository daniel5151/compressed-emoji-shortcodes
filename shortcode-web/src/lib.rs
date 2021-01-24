use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn lookup(s: &str) -> Option<String> {
    maximally_compressed_emoji_shortcodes::lookup(s).map(Into::into)
}
