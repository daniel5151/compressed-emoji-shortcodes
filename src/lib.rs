#![no_std]

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub fn lookup<'a>(s: &str, buf: &'a mut [u8; 8]) -> Option<&'a str> {
    let raw = *EMOJI.get(s)?;
    let bytes = raw.to_le_bytes();
    let leading_zeros = bytes.iter().position(|b| *b != 0)?;
    let len = 8 - leading_zeros;
    buf[..len].copy_from_slice(&bytes[leading_zeros..]);
    let s = unsafe { core::str::from_utf8_unchecked(&buf[..len]) };
    Some(s)
}
