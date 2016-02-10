# minilzo-rs - A wrapper around minilzo, the lightweight subset of the LZO library

[![Build Status](https://travis-ci.org/badboy/minilzo.svg?branch=master)](https://travis-ci.org/badboy/minilzo)
[![crates.io](http://meritbadge.herokuapp.com/minilzo)](https://crates.io/crates/minilzo)

LZO is a compression library with focus on decompression speed.
originally implemented by Markus F.X.J. Oberhumer.
minilzo is a lightweight subset of the full LZO library.

It is available [online as a C library](http://www.oberhumer.com/opensource/lzo/#minilzo).

This rust library is a wrapper around the minilzo library
and is fully compatible with LZO/minilzo compressed data.

## Build

```
cargo build --release
```

## Usage

```rust
use minilzo;

fn main() {
    let data = b"foobar";

    let compressed = minilzo::compress(&data[..]).unwrap();

    let decompressed = minilzo::decompress(&compressed, data.len()).unwrap();
}
```

## Tests

Run tests with:

```
cargo test
```

Run benchmarks with:

```
cargo bench
```

## License

The `minilzo-rs` wrapper library is licensed under the terms of the [MIT License](LICENSE).  
LZO itself is licensed under the terms of the [GNU General Public License](http://www.oberhumer.com/opensource/gpl.html) (GPL v2+).
