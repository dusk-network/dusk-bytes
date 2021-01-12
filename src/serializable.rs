// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

pub trait BadLength {
    fn bad_length(found: usize, expected: usize) -> Self;
}

pub trait InvalidChar {
    fn invalid_char(index: usize) -> Self;
}

pub trait Serializable<const N: usize> {
    type Error;
    const SIZE: usize = N;

    fn from_bytes(buf: &[u8; N]) -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn to_bytes(&self) -> [u8; N];
}

pub trait DeserializableSlice<const N: usize>: Serializable<N> {
    fn from_bytes_slice(buf: &[u8]) -> Result<Self, Self::Error>
    where
        Self: Sized,
        Self::Error: BadLength,
    {
        if buf.len() < N {
            Err(Self::Error::bad_length(buf.len(), N))
        } else {
            let mut bytes = [0u8; N];
            (&mut bytes[..N]).copy_from_slice(&buf[..N]);
            Self::from_bytes(&bytes)
        }
    }
}

pub trait DeserializableHexStr<const N: usize>: Serializable<N> {
    fn from_hex_str(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized,
        Self::Error: InvalidChar + BadLength,
    {
        let expected = N * 2;
        if s.len() < expected {
            Err(Self::Error::bad_length(s.len(), expected))
        } else {
            let mut bytes = [0u8; N];
            let s = s.as_bytes();

            for i in (0..expected).step_by(2) {
                let n: u8 = match (val(s[i]), val(s[i + 1])) {
                    (Some(h), Some(l)) => (h << 4) + l,
                    (_, _) => return Err(Self::Error::invalid_char(i)),
                };
                bytes[i / 2] = n;
            }

            Self::from_bytes(&bytes)
        }
    }
}

fn val(c: u8) -> Option<u8> {
    match c {
        b'A'..=b'F' => Some(c - b'A' + 10),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'0'..=b'9' => Some(c - b'0'),
        _ => None,
    }
}
