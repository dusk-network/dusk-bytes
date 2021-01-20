// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use derive_hex::HexDebug;

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

#[test]
fn lower_hex() {
    let beef = Beef {};

    assert_eq!(format!("{:x}", beef), "beef");
}

#[test]
fn lower_hex_alt() {
    let beef = Beef {};

    assert_eq!(format!("{:#x}", beef), "0xbeef");
}

#[test]
fn upper_hex() {
    let beef = Beef {};

    assert_eq!(format!("{:X}", beef), "BEEF");
}

#[test]
fn upper_hex_alt() {
    let beef = Beef {};

    assert_eq!(format!("{:#X}", beef), "0xBEEF");
}
