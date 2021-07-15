// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use super::errors::{BadLength, InvalidChar};
use super::serialize::Serializable;

/// An optional trait used to parse a string slice for types that implements
/// the [`Serializable`] trait.
/// The default implementation makes use of [`Serializable`] trait to provide
/// the necessary parsing functionality without additional code from the
/// consumer.
pub trait ParseHexStr<const N: usize>: Serializable<N> {
    /// Parse a string slice as bytes hex representation and returns `
    fn from_hex_str(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized,
        Self::Error: BadLength + InvalidChar,
    {
        let expected = N * 2;
        if s.len() < expected {
            return Err(Self::Error::bad_length(s.len(), expected));
        }

        let mut bytes = [0u8; N];
        let s = s.as_bytes();

        for i in (0..expected).step_by(2) {
            let n: u8 = match (val(s[i]), val(s[i + 1])) {
                (Some(h), Some(l)) => (h << 4) + l,
                (None, _) => {
                    return Err(Self::Error::invalid_char(s[i].into(), i))
                }
                (_, None) => {
                    return Err(Self::Error::invalid_char(
                        s[i + 1].into(),
                        i + 1,
                    ))
                }
            };
            bytes[i / 2] = n;
        }

        Self::from_bytes(&bytes)
    }
}

/// A constant funtion to parse a bytes string representing hexadecimals
/// (e.g. `b"fe12c6"` ) into bytes (e.g `[0xfe, 0x12, 0xc6]`).
/// If a smaller destination buffer is provided, the value will be truncated
/// (e.g `[0xfe, 0x12]`); if a bigger destination buffer is provided, it will
/// be padded with zeroes (e.g. `[0xfe, 0x12, 0xc6, 0x0, 0x0])
///
/// If an invalid character is given, it will panic at compile time.
pub const fn hex<const N: usize, const M: usize>(bytes: &[u8; N]) -> [u8; M] {
    let mut buffer = [0u8; M];

    let mut i = 0;
    let mut j = 0;
    while i < N && j < M {
        let n = match (val(bytes[i]), val(bytes[i + 1])) {
            (Some(h), Some(l)) => (h << 4) + l,
            (_, _) => panic!("hex(): failed to parse the input as hex number"),
        };

        buffer[j] = n;
        i += 2;
        j += 1;
    }
    buffer
}

const fn val(c: u8) -> Option<u8> {
    match c {
        b'A'..=b'F' => Some(c - b'A' + 10),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'0'..=b'9' => Some(c - b'0'),
        _ => None,
    }
}

// Auto trait [`ParseHexStr`] for any type that implements [`Serializable`]
impl<T, const N: usize> ParseHexStr<N> for T where T: Serializable<N> {}
