# derive-hex

`derive-hex` provides two small derive macros that implement hex formatting for
types that expose a `to_bytes()` method.

This crate is primarily meant to be used alongside the [`dusk-bytes`] crate
where `to_bytes()` is provided by the `Serializable` trait.

## Derives

`#[derive(Hex)]` implements `core::fmt::LowerHex` (`{:x}` / `{:#x}`) and `core::fmt::UpperHex` (`{:X}` / `{:#X}`).

`#[derive(HexDebug)]` includes everything from `Hex` and additionally implements `core::fmt::Debug` and formats using hex when you use the debug formatter flags (`{:x?}` / `{:#x?}` / `{:X?}` / `{:#X?}`).

Both derives format the output by iterating over `self.to_bytes()` and writing each byte as two hexadecimal digits.

## Example

```rust
use derive_hex::HexDebug;

#[derive(Copy, Clone, PartialEq, Eq, HexDebug)]
struct IdPrefix([u8; 4]);

impl IdPrefix {
    // The derives only require a `to_bytes()` method.
    pub fn to_bytes(&self) -> [u8; 4] {
        self.0
    }
}

let p = IdPrefix([0xde, 0xad, 0xbe, 0xef]);

assert_eq!(format!("{:x}", p), "deadbeef");
assert_eq!(format!("{:#x}", p), "0xdeadbeef");
assert_eq!(format!("{:X}", p), "DEADBEEF");

// `HexDebug` also wires debug formatter flags (`{:x?}` / `{:X?}`) to hex.
assert_eq!(format!("{:x?}", p), "deadbeef");
assert_eq!(format!("{:#X?}", p), "0xDEADBEEF");
```

## License

Licensed under the Mozilla Public License 2.0 (MPL-2.0).
