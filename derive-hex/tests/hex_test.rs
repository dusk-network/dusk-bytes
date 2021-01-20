// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use derive_hex::HexDebug;
use dusk_bytes::{Error, Serializable};

#[derive(HexDebug)]
struct Beef {}

impl Serializable<2> for Beef {
    type Error = Error;
    fn from_bytes(buf: &[u8; Self::SIZE]) -> Result<Self, Error> {
        if buf[0] == 0xbe && buf[1] == 0xef {
            Ok(Self {})
        } else {
            Err(Error::InvalidData)
        }
    }

    fn to_bytes(&self) -> [u8; Self::SIZE] {
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
