#![no_std]

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub fn lookup<'a>(s: &str) -> Option<&'a str> {
    for &(k, v) in FIXUP_TABLE.iter() {
        if s == k {
            return Some(v);
        }
    }

    for map in EMOJI_MAPS {
        if let Some(s) = map.get(s) {
            return Some(s);
        }
    }
    None
}
