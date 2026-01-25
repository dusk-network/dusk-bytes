// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use dusk_bytes::{BadLength, InvalidChar, Serializable};

use dusk_bytes::HexDebug;
#[derive(HexDebug)]
pub struct Beef {}

#[derive(Debug)]
pub enum BeefError {
    InvalidBytes,
    UnexpectedEof,
    #[allow(dead_code)]
    CharNotValid(char, usize),
}

impl Serializable<2> for Beef {
    type Error = BeefError;
    fn from_bytes(buf: &[u8; Self::SIZE]) -> Result<Self, Self::Error> {
        if buf[0] == 0xbe && buf[1] == 0xef {
            Ok(Self {})
        } else {
            Err(BeefError::InvalidBytes)
        }
    }

    fn to_bytes(&self) -> [u8; Self::SIZE] {
        [0xbe, 0xef]
    }
}

// Implementing DeserializableSlice requires `Error` to implements `BadLength`
// too
impl BadLength for BeefError {
    fn bad_length(_found: usize, _expected: usize) -> Self {
        Self::UnexpectedEof
    }
}

// Implementing ParseHexStr requires `Error` to implements `InvalidChar` too
impl InvalidChar for BeefError {
    fn invalid_char(ch: char, index: usize) -> Self {
        Self::CharNotValid(ch, index)
    }
}
