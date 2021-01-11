# Maximally Compressed Github Emoji Shortcodes

_Alt Title: Why Perfect Hash Functions are Magic_

**NOTE:** This is still a WIP. I'm still hacking around with crunching things down even more (e.g: instead of storing emoji as strings, I'll store them in a compacted `u64`-backed representation). Oh, and the writeup still needs some work as well, obviously.

## Building and Running

At the moment, building this library requires running a \*nix OS with `curl` installed (to download the emoji database as part of the build process).

Aside from that, this is a bog-standard Rust library, which can be built with `cargo build`.

There are several test binaries included in this repo:

- `collision-test`: a very messy playground for testing how well the library rejects invalid inputs.
- `example_no_std`: a `no_std` Rust binary that serves as a rough benchmark for how much space the library occupies in a final binary.
- `shortcode-web`: using the magic of `wasm-pack`, you can play with this project via an [incredibly barebones and terrible looking] online demo!

Try out the online demo [here](https://prilik.com/compressed-emoji-shortcodes)!

Note that the `.wasm` size of the `shortcode-web` demo is not representative of the binary size on an proper embedded platform, since `wasm-bindgen` introduces almost 14kb of overhead for some reason (i.e: when the single exported function is replaced with a noop). I could _probably_ slim this down by bypassing `wasm-bindgen` entirely, and figuring out how to accept Javascript `String`s over the FFI, but that's _high effort_. So yeah, just subtract 14kb from the (uncompressed) `.wasm` size to get a better idea of the compression factor.

## Inspiration

I'm a big fan of custom mechanical keyboards, as aside from looking and sounding awesome, they're also a really fun hardware-hacking platform. Most custom mechanical keyboards use the open source [QMK firmware](https://github.com/qmk/qmk_firmware), which comes with a host of built-in features for doing just about anything you'd want on a keyboard.

One such feature is the ability to [type Unicode characters](https://beta.docs.qmk.fm/using-qmk/software-features/feature_unicode). In a nutshell, you can program the keyboard such that when a certain key (or sequence of keys) is pressed, the keyboard will send whatever keystrokes are required to input Unicode on any of the several [supported platforms](https://beta.docs.qmk.fm/using-qmk/software-features/feature_unicode#2-input-modes-id-input-modes). This is great, because it means that the 🅱 emoji is never more than a few keystrokes away!

One key limitation of QMK's built-in unicode support is that it requires users to manually specify what set of characters they'd like to support. Of course, this makes perfect sense when you consider that most keyboards use low-cost, low-power MCUs with very limited amounts of Flash ROM. I mean, come on, it's not like it'd be even _remotely_ feasible to stuff an entire unicode/emoji database on chip, right?

Well, maybe it is!

This project is an experiment to try and find the maximally compressed representation of the entire [github shortcode](https://www.webfx.com/tools/emoji-cheat-sheet/) set. Or, in other words, what's the smallest amount of storage (code and data) required to write a function with the following signature:

```rust
fn shortcode_to_emoji(shortcode: String) -> Emoji // (where `Emoji` can be somehow converted back into a UTF-8 string)
```

Oh, and of course, we want this to run on platforms without heap allocation, so everything _must_ be pre-computed at compile time.

## Final Implementation

This library uses a modified version of the incredible [rust-phf](https://github.com/sfackler/rust-phf) to generate a [perfect](https://en.wikipedia.org/wiki/Perfect_hash_function) hash-map of shortcodes to emoji at compile time, but instead of storing the keys as raw strings (which would take up a _lot_ of space, around `size_of(char*)` + 5 or 6 bytes on average), the map only stores a _1 byte hash_ of the expected string. This _substantially_ reducing the storage requirements, at the expense of the potential "false positive" results.

Additionally, instead of storing emoji as raw UTF-8 strings (which would incur a totally unreasonable `size_of(char*) + strlen(s)` overhead per emoji - or roughly \~) strings are stored inline as the bytes of a `u64`.

> Note: At the time of writing, using `u64`s as the backing representation limits what shortcodes can be represented (i.e: only emoji with a UTF-8 encoding less than 9 bytes). See `oversized_emoji.txt` for a list of missing shortcodes.

Lastly, instead of storing map entries as tuples in a `entries: &[(u8, u64)]`, keys and values are split into separate `keys: &[u8]`, `values: &[u64]` arrays, which eliminates a _lot_ of padding bytes.

## Iteration and Further Ideas

TODO: write some more about the process of arriving at the final solution

- Support storing any length of emoji by using "cascading" PHFs?
    - i.e: have `n` perfect hash tables for all `n` in `emoji.map(|e| e.as_bytes().len()).minmax()`.
    - this would improve data density, as there wouldn't be all the wasted space from all the leading zeros for emoji that don't need them.

## Miscellaneous Thoughts

> I might throw these together into a proper writeup at some point. Or I might not - I'm not sure yet.

Why does a PHF even need to store the keys in the first place? After all, won't every input just automagically map to the correct value?

Well, while that may be true for _valid_ inputs, plugging an _invalid_ input into a PHF will just return some arbitrary index. To reject invalid inputs, a perfect hash function will include an [additional check](https://github.com/sfackler/rust-phf/blob/9b70bd9/phf/src/map.rs#L88) that compares the input key against the expected key at that particular index. This will ensure, with 100% certainty, that an invalid input is rejected, and doesn't return a bogus value.

If this extra check wasn't in place, just imagine what sort of chaos simple input errors might result in! A harmless spelling mistake while typing `:smile:`, compounded with spectacularly bad statistical chance, somehow results in accidentally typing `:eggplant:` instead. All of a sudden, a harmless DM to a co-worker turns into a meeting with HR where you're frantically trying to explain how perfect hash functions work. Yikes :eggplant:

Clearly, we need _some_ way to reject these sorts of "false positives", but at the same time, we _really_ don't want to store a massive set of strings in ROM.

That's where the hash comes in: instead of storing the strings in their entirety, we instead take a 1-byte hash of the strings, and use that hash to reject false positives!

Now, I'm no statistician, so I really can't comment to the theoretical accuracy of this approach. All I can say is that _empirically_, both through everyday use and more aggressive stress testing via the `collision-test` test harness, 1 byte seems to work _exceptionally well_.

## Acknowledgements

This project wouldn't have been possible without the incredible [rust-phf](https://github.com/sfackler/rust-phf) library. The in-tree version of `rust-phf` is a stripped down version and heavily modified version the library, optimizing the map's internal representation for this particular use-case.

The initial POC of this project was based off of https://github.com/kornelski/gh-emoji.

The emoji shortcode database is downloaded directly from Github's [gemoji](https://github.com/github/gemoji/tree/master) library.
