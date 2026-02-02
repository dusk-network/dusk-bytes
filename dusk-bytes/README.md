# dusk-bytes

`dusk-bytes` is a small, `no_std` friendly crate that helps you implement
fixed-size (de)serialization for your types using const generics.

A type that can be represented by exactly `N` bytes implements [`Serializable<N>`]. From there, the crate provides convenience traits to:

- deserialize from slices and byte readers (`DeserializableSlice`).
- parse hex strings (`ParseHexStr`).
- parse hex literals at compile time (`hex()`).
- and format types as hex (`Hex` / `HexDebug`).

This crate is used as the foundation for a number of Dusk types where a
compact, allocation-free byte representation is desirable.

## Features

- `#![no_std]` (no `alloc` required).
- Const-generic byte sizes (e.g. `Serializable<32>`).
- Default helpers that work with custom error types via the `BadLength` and
  `InvalidChar` traits.
- Built-in `Serializable` implementations for common integer primitives
  (little-endian).

## Quick start

Implement [`Serializable<N>`] for your type:

```rust
use dusk_bytes::{DeserializableSlice, ParseHexStr, Serializable};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

// `DeserializableSlice` is auto-implemented for any `Serializable` type.
let p = Point::from_slice(&[1, 0, 2, 0]).unwrap();
assert_eq!(p, Point { x: 1, y: 2 });

// `ParseHexStr` is also auto-implemented.
let p2 = Point::from_hex_str("01000200").unwrap();
assert_eq!(p2, p);
```

## Hex parsing

### Runtime: `ParseHexStr::from_hex_str`

`from_hex_str` parses the first `N * 2` characters of a string slice (two hex
characters per byte).

- If the string is shorter than `N * 2`, it returns a `BadLength` error.
- If a non-hex character is found, it returns an `InvalidChar` error.
- If the string is longer, extra characters are ignored.

### Compile-time: `hex()`

`hex()` is a `const fn` that parses an ASCII hex byte string like
`b"deadbeef"` into a byte array.

```rust
use dusk_bytes::hex;

const MAGIC: [u8; 4] = hex(b"deadbeef");
assert_eq!(MAGIC, [0xde, 0xad, 0xbe, 0xef]);
```

The input byte string must have an even length (two hex digits per output
byte). Invalid characters cause a compile-time panic during const evaluation.

## Hex formatting: `Hex` and `HexDebug`

`dusk-bytes` re-exports two derive macros from the companion `derive-hex` crate:

- `#[derive(Hex)]` implements `core::fmt::LowerHex` and `core::fmt::UpperHex`.
- `#[derive(HexDebug)]` additionally implements `core::fmt::Debug` and formats
  the value as hex for `{:x?}` / `{:X?}`.

Both derives expect your type to expose a `to_bytes()` method (which the
[`Serializable`] trait already provides).

```rust
use dusk_bytes::HexDebug;

#[derive(Copy, Clone, HexDebug)]
struct IdPrefix([u8; 4]);

impl IdPrefix {
    pub fn to_bytes(&self) -> [u8; 4] {
        self.0
    }
}

let p = IdPrefix([0xde, 0xad, 0xbe, 0xef]);
assert_eq!(format!("{:x}", p), "deadbeef");
assert_eq!(format!("{:#x}", p), "0xdeadbeef");
assert_eq!(format!("{:x?}", p), "deadbeef");
```

## Readers and writers

For embedded / `no_std` environments, the crate provides minimal `Read` / `Write`
traits (inspired by `std::io`) and implements them for:

- `&[u8]` (reader)
- `&mut [u8]` (writer)

```rust
use dusk_bytes::{DeserializableSlice, Read, Serializable, Write};

let value: u32 = 0x01020304;

let mut buf = [0u8; 4];
{
    let mut w = &mut buf[..];
    w.write(&value.to_bytes()).unwrap();
}

let mut r = &buf[..];
let parsed = u32::from_reader(&mut r).unwrap();
assert_eq!(parsed, value);
assert!(r.is_empty());
```

## Error handling

The crate provides a small default [`Error`] enum that is used by the built-in
primitive implementations.

If you want to keep your own error type, implement:

- [`BadLength`] (for slice/reader underflow), and
- [`InvalidChar`] (for hex parsing).

Those traits are used by the default implementations of `DeserializableSlice`
and `ParseHexStr`.

## License

Licensed under the Mozilla Public License 2.0 (MPL-2.0).
