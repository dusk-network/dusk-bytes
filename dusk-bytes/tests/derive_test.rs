// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

#![cfg(feature = "derive")]

use dusk_bytes::{Hex, HexDebug};

#[derive(Hex)]
struct Cafe;

impl Cafe {
    fn to_bytes(&self) -> [u8; 2] {
        [0xca, 0xfe]
    }
}

#[derive(HexDebug)]
struct Beef;

impl Beef {
    fn to_bytes(&self) -> [u8; 2] {
        [0xbe, 0xef]
    }
}

#[test]
fn derive_feature_reexports_hex_debug() {
    assert_eq!(format!("{:X}", Cafe), "CAFE");
    assert_eq!(format!("{:x}", Beef), "beef");
    assert_eq!(format!("{:X?}", Beef), "BEEF");
}
