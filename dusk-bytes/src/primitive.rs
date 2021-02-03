// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use crate::{Error, Serializable};

macro_rules! impl_serializable {
    ($ty:ty) => {
        impl Serializable<{ core::mem::size_of::<$ty>() }> for $ty {
            type Error = Error;

            fn from_bytes(buf: &[u8; Self::SIZE]) -> Result<Self, Self::Error> {
                Ok(Self::from_le_bytes(*buf))
            }

            fn to_bytes(&self) -> [u8; Self::SIZE] {
                <$ty>::to_le_bytes(*self)
            }
        }
    };
}

impl_serializable!(u16);
impl_serializable!(u32);
impl_serializable!(u64);
impl_serializable!(u128);
