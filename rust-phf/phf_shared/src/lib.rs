#![no_std]

use core::hash::Hasher;
use core::num::Wrapping;
use siphasher::sip128::{Hash128, Hasher128, SipHasher13};

#[non_exhaustive]
pub struct Hashes {
    pub g: u32,
    pub f1: u32,
    pub f2: u32,
}

/// A central typedef for hash keys
///
/// Makes experimentation easier by only needing to be updated here.
pub type HashKey = u64;

#[inline]
pub fn displace(f1: u32, f2: u32, d1: u32, d2: u32) -> u32 {
    (Wrapping(d2) + Wrapping(f1) * Wrapping(d1) + Wrapping(f2)).0
}

/// `key` is from `phf_generator::HashState`.
#[inline]
pub fn hash<T: ?Sized + PhfHash>(x: &T, key: &HashKey) -> Hashes {
    let mut hasher = SipHasher13::new_with_keys(0, *key);
    x.phf_hash(&mut hasher);

    let Hash128 {
        h1: lower,
        h2: upper,
    } = hasher.finish128();

    Hashes {
        g: (lower >> 32) as u32,
        f1: lower as u32,
        f2: upper as u32,
    }
}

#[inline]
pub fn checksum(s: &str) -> u16 {
    s.as_bytes()
        .iter()
        .copied()
        // .fold(0, |a, x| a.wrapping_add(x)) // very easy to get collision
        .fold(0, |a, x| (a.wrapping_mul(37).wrapping_add(x as _)))
}

/// Return an index into `phf_generator::HashState::map`.
///
/// * `hash` is from `hash()` in this crate.
/// * `disps` is from `phf_generator::HashState::disps`.
/// * `len` is the length of `phf_generator::HashState::map`.
#[inline]
pub fn get_index(hashes: &Hashes, disps: &[(u32, u32)], len: usize) -> u32 {
    let (d1, d2) = disps[(hashes.g % (disps.len() as u32)) as usize];
    displace(hashes.f1, hashes.f2, d1, d2) % (len as u32)
}

/// A trait implemented by types which can be used in PHF data structures.
///
/// This differs from the standard library's `Hash` trait in that `PhfHash`'s
/// results must be architecture independent so that hashes will be consistent
/// between the host and target when cross compiling.
pub trait PhfHash {
    /// Feeds the value into the state given, updating the hasher as necessary.
    fn phf_hash<H: Hasher>(&self, state: &mut H);

    /// Feeds a slice of this type into the state provided.
    fn phf_hash_slice<H: Hasher>(data: &[Self], state: &mut H)
    where
        Self: Sized,
    {
        for piece in data {
            piece.phf_hash(state);
        }
    }
}

impl PhfHash for str {
    #[inline]
    fn phf_hash<H: Hasher>(&self, state: &mut H) {
        self.as_bytes().phf_hash(state)
    }
}

impl PhfHash for [u8] {
    #[inline]
    fn phf_hash<H: Hasher>(&self, state: &mut H) {
        state.write(self);
    }
}

impl<'a, T: 'a + PhfHash + ?Sized> PhfHash for &'a T {
    fn phf_hash<H: Hasher>(&self, state: &mut H) {
        (*self).phf_hash(state)
    }
}
