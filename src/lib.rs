#![no_std]

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub fn lookup(s: &str) -> Option<&'static str> {
    EMOJI.get(s).copied()
}
