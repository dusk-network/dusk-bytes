// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

pub trait BadLength {
    fn bad_length(found: usize, expected: usize) -> Self;
}

pub trait Serializable<const N: usize> {
    type Error: BadLength;
    const SIZE: usize = N;

    fn from_bytes(buf: &[u8]) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        if buf.len() < Self::SIZE {
            Err(Self::Error::bad_length(buf.len(), Self::SIZE))
        } else {
            Self::from_bytes_unsized(buf)
        }
    }

    fn from_bytes_unsized(buf: &[u8]) -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn to_bytes(&self) -> [u8; N];
}
