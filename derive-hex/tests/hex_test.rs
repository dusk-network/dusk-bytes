// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use derive_hex::{Hex, HexDebug};

#[derive(HexDebug)]
struct Beef {}

// Cannot import as dev-dependencies `dusk-bytes` since it creates a circular
// dependendecies that `cargo publish` can't understand is legit, until
// `cargo publish --all` is implemented (or similar solution).
//
// So we manually add the `to_bytes` method requested by `HexDebug` macro.
impl Beef {
    pub fn to_bytes(&self) -> [u8; 2] {
        [0xbe, 0xef]
    }
}

#[derive(Hex)]
struct GenericHex<T>
where
    T: Copy + Into<u8>,
{
    value: T,
}

impl<T> GenericHex<T>
where
    T: Copy + Into<u8>,
{
    fn to_bytes(&self) -> [u8; 1] {
        [self.value.into()]
    }
}

#[derive(HexDebug)]
struct GenericHexDebug<T>
where
    T: Copy + Into<u8>,
{
    value: T,
}

impl<T> GenericHexDebug<T>
where
    T: Copy + Into<u8>,
{
    fn to_bytes(&self) -> [u8; 1] {
        [self.value.into()]
    }
}

#[test]
fn formatting() {
    let beef = Beef {};

    assert_eq!(format!("{:x}", beef), "beef");
    assert_eq!(format!("{:#x}", beef), "0xbeef");
    assert_eq!(format!("{:X}", beef), "BEEF");
    assert_eq!(format!("{:#X}", beef), "0xBEEF");
    assert_eq!(format!("{:x?}", beef), "beef");
    assert_eq!(format!("{:#x?}", beef), "0xbeef");
    assert_eq!(format!("{:X?}", beef), "BEEF");
    assert_eq!(format!("{:#X?}", beef), "0xBEEF");
}

#[test]
fn generic_derives_preserve_bounds() {
    let hex = GenericHex { value: 0xabu8 };
    let debug = GenericHexDebug { value: 0xcdu8 };

    assert_eq!(format!("{hex:x}"), "ab");
    assert_eq!(format!("{debug:?}"), "cd");
    assert_eq!(format!("{debug:X?}"), "CD");
}
