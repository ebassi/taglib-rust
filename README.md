## TagLib-Rust  [![Build Status][trav-ci-img]][trav-ci]

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

You need [TagLib](http://taglib.org/) installed on your system to build. This can be found in the following packages:

- Arch Linux: [taglib](https://www.archlinux.org/packages/extra/x86_64/taglib/)
- CentOS/Fedora: taglib
- Gentoo: [media-libs/taglib](https://packages.gentoo.org/packages/media-libs/taglib)
- Ubuntu/Debian: [libtagc0-dev](https://packages.debian.org/search?searchon=names&keywords=libtagc0-dev)

The `pkg-config` Rust module can also be optionally used to find the location
of the TagLib library when building.

### Using TagLib-Rust

If you're using [cargo][crates] to manage your project, you can download
through Crates.io:

```toml
[dependencies]
taglib = "*"
```

Alternatively, you can pull it from [GitHub][taglib-gh]:

```rust
[dependencies]
taglib = { git = "https://github.com/ebassi/taglib-rust" }
```

Otherwise, clone [the Git repository][taglib-gh], and run [cargo][crates]:

```
$ cargo build
```

### Examples

TagLib-Rust comes with some examples on how to use it, see the `examples`
directory in the [the Git repository][taglib-gh].

### Documentation

The API reference for TagLib-Rust is available [online][taglib-docs].

## License

TagLib-Rust is licensed under the terms of the [MIT/X11 license][osi-mit].

See the `LICENSE` file for more details.


[trav-ci-img]: https://travis-ci.org/ebassi/taglib-rust.png?branch=master
[trav-ci]: https://travis-ci.org/ebassi/taglib-rust
[crates]: http://crates.io/
[taglib-gh]: https://github.com/ebassi/taglib-rust
[taglib-docs]: http://ebassi.github.io/taglib-rust/docs/taglib/
[osi-mit]: http://opensource.org/licenses/MIT
