# dusk-bytes

This repository is a small Rust workspace containing two crates:

- **`dusk-bytes`**: a `no_std` friendly set of traits for fixed-size
  (de)serialization using const generics, plus hex parsing helpers.
- **`derive-hex`**: companion derive macros (`Hex`, `HexDebug`) that implement
  `LowerHex` / `UpperHex` formatting (and optionally `Debug`) for types that
  expose a `to_bytes()` method (for example types implementing
  `dusk_bytes::Serializable`).

## Crates

### `dusk-bytes`

The core abstraction is `Serializable<const N: usize>`, which converts between a type and a fixed-size byte array `[u8; N]`.

### `derive-hex`

Derive macros to print a type as hex. `#[derive(Hex)]` implements `core::fmt::LowerHex` and `core::fmt::UpperHex`. `#[derive(HexDebug)]` also implements `core::fmt::Debug` so `{:x?}` / `{:X?}` prints in hex.

## Example

```rust
use dusk_bytes::{DeserializableSlice, ParseHexStr, Serializable};
use dusk_bytes::HexDebug;

#[derive(Copy, Clone, Debug, PartialEq, Eq, HexDebug)]
struct Point {
    x: u16,
    y: u16,
}

impl Serializable<4> for Point {
    type Error = dusk_bytes::Error;

    fn from_bytes(buf: &[u8; 4]) -> Result<Self, Self::Error> {
        let x = u16::from_le_bytes([buf[0], buf[1]]);
        let y = u16::from_le_bytes([buf[2], buf[3]]);
        Ok(Self { x, y })
    }

    fn to_bytes(&self) -> [u8; 4] {
        let mut out = [0u8; 4];
        out[0..2].copy_from_slice(&self.x.to_le_bytes());
        out[2..4].copy_from_slice(&self.y.to_le_bytes());
        out
    }
}

let p = Point::from_hex_str("01000200").unwrap();
assert_eq!(p, Point { x: 1, y: 2 });

// Hex formatting is provided by `#[derive(HexDebug)]`.
assert_eq!(format!("{:x}", p), "01000200");

// `DeserializableSlice` is auto-implemented for types implementing
// `Serializable`.
let bytes = [1u8, 0, 2, 0];
let p2 = Point::from_slice(&bytes).unwrap();
assert_eq!(p2, p);
```

## License

Licensed under the Mozilla Public License 2.0 (MPL-2.0). See [LICENSE](./LICENSE).
