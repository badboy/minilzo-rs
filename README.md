# minilzo-rs - A wrapper around minilzo, the lightweight subset of the LZO library


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
