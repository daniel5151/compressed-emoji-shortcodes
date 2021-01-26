# Highly Compressed Emoji Shortcode Mapping

An experiment to try and find a highly compressed representation of the
entire
[unicode shortcodes-to-emoji](https://raw.githubusercontent.com/github/gemoji/master/db/emoji.json)
mapping that can be indexed _without requiring any dynamic allocation_. In other
words: what's the smallest amount of static storage(code and data) required to
write a function with the following signature:

```rust
fn shortcode_to_emoji(input: &str) -> Option<&str>
```

> **DISCLAIMER:** This is a project that was hacked together over the course of
> a couple weekends, where iteration speed took priority over code
> quality/robustness. The implementation is a mess, and the build system is
> incredibly brittle. If you're even _remotely_ considering building off this
> repo - please don't. You'd be much better off building off these _ideas_ and
> writing a new implementation from scratch.

Make sure to try out the
[online demo](https://prilik.com/compressed-emoji-shortcodes)!

## Building and Running

At the moment, building this library requires running a \*nix OS with `curl`
installed, which is required to download the emoji database used as part of the
build process. Aside from that, the core `maximally-compressed-emoji-shortcodes`
library is a bog-standard Rust crate which can be built with `cargo build`.

Given that this is more of a quick-and-dirty experiment rather than a proper
ready-to-use library, there isn't any easy-to-use config file to play around
with. If you're seriously interested in playing around with this mess of code,
get ready to do some spelunking. Some potentially useful starting off points:

-   Changing the size of the hash - `./rust-phf/phf/src/map.rs:18` (the
    `keys: Slice<u16>` field) and `./rust-phf/phf_shared/src/lib.rs:43` (the
    `checksum` function).
-   Fixup Table search times - `build.rs`
-   Swapping out the key/value data set - `build.rs`

This repo includes several test/example projects that use the
`maximally-compressed-emoji-shortcodes` library:

-   `kowalski-analysis`: a _very_ messy playground for testing various
    properties of the library (e.g: false-positive rates, accuracy, compression
    ratio, etc...).
-   `example_no_std`: a `no_std` Rust binary that serves as a rough benchmark
    for how much space the library will occupy when deployed in an embedded
    context.
-   `shortcode-web`: using the magic of `wasm-pack`, you can play with this
    project via a neat little online demo! Try it out at
    [here](https://prilik.com/compressed-emoji-shortcodes)!
    -   _Note:_ the `.wasm` size of the `shortcode-web` demo is not
        representative of the binary size on an proper embedded platform, since
        `wasm-bindgen` introduces almost 15kb of overhead for some reason (i.e:
        when the single exported function is replaced with a noop). I could
        probably slim this down by bypassing `wasm-bindgen` entirely, and
        figuring out how to accept Javascript `String`s over the FFI, but that's
        _high effort_. So yeah, just subtract 15kb from the (uncompressed)
        `.wasm` size to get a better idea of the compression factor.

## Future Work?

I'm pretty much done with this project for now, but there are still a few ideas
that might be worth exploring to compress things down even more:

-   Compressing the emoji UTF-8 strings using some sort of domain specific
    representation
    -   e.g: it seems like most emoji fall under a
        [small subrange](https://stackoverflow.com/questions/30470079/emoji-value-range)
        of unicode codepoints, it might be possible to shave a couple bits of
        overhead from each emoji mapping by adding/removing an offset from the
        stored value.
-   Using non-standard key hash sizes (i.e: 9-bit, 10-bit, etc...).
    -   Follow up: automatically trying out various key hash sizes to minimize
        space overhead while maintaining a favorable hash-collision rate.
-   Actually cleaning up this abomination of a codebase and releasing a proper
    library that employs these various techniques

Oh, and of course, I should probably port it to
[one of my keyboards](https://github.com/daniel5151/qmk_firmware). After all,
that was the whole inspiration for this endeavor!

## Acknowledgements

This project wouldn't have been possible without the incredible
[rust-phf](https://github.com/sfackler/rust-phf) library. The in-tree version of
`rust-phf` is a stripped down version and heavily modified version the library,
optimizing the map's internal representation for this particular use-case.

The initial POC of this project was based off of
https://github.com/kornelski/gh-emoji.

The emoji shortcode database is downloaded directly from Github's
[gemoji](https://github.com/github/gemoji/tree/master) library.

Special thanks to [Matt D'Souza](https://github.com/DSouzaM) and
[Ethan Hardy](https://github.com/ethan-hardy), who I nerd-sniped into helping me
with this funky little project.
