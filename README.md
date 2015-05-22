## TagLib-Rust

Simple bindings for TagLib in Rust

## Overview

TagLib-Rust is a library that allows accessing audio meta-data in Rust, by
using the TagLib library.

TagLib-Rust supports reading and editing the meta-data of several popular
audio formats. Currently there is support for:

 * ID3v1 and ID3v2 for MP3 files
 * Ogg Vorbis and FLAC
 * MPC
 * Speex
 * WavPack
 * TrueAudio
 * MP4
 * ASF

### Requirements

TagLib-Rust requires:

 * Rust 1.0
 * TagLib
 * the libc Rust module

It also optionally depends on the `pkg-config` Rust module to find the
location of the TagLib library when building.

### Using TagLib-Rust

If you're using [cargo][crates] to manage your project, you can download
through Crates.io:

```toml
    [dependencies]
    taglib = "0.1.0"
```

Alternatively, you can pull it from [GitHub][taglib-gh]:

```rust
    [dependencies.taglib]
    git = "https://github.com/ebassi/taglib-rust"
```

Otherwise, clone [the Git repository][taglib-gh], and run [cargo][crates]:

    $ cargo build

### Examples



## License

TagLib-Rust is licensed under the terms of the Lesser General Public
License version 2.1, or, at your option under the terms of the Mozilla
Public License version 1.1.

 * LGPL v2.1: http://www.gnu.org/licenses/
 * MPL 1.1: https://www.mozilla.org/MPL/1.1/index.txt

[trav-ci-img]: https://travis-ci.org/ebassi/taglib-rust.png?branch=master
[trav-ci]: https://travis-ci.org/ebassi/taglib-rust
[crates]: http://crates.io/
[taglib-gh]: https://github.com/ebassi/taglib-rust
