// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use super::errors::BadLength;

/// The core trait used to implement [`from_bytes`] and [`to_bytes`]
pub trait Serializable<const N: usize> {
    /// The size of
    const SIZE: usize = N;
    /// The type returned in the event of a conversion error.
    type Error;

    /// Deserialize a [`&[u8; N]`] into [`Self`], it might be fail.
    fn from_bytes(buf: &[u8; N]) -> Result<Self, Self::Error>
    where
        Self: Sized;

    /// Serialize [`Self`] into a [`[u8; N]`].
    fn to_bytes(&self) -> [u8; N];
}

/// An optional trait used to implement [`from_slice`] on top of types that
/// uses [`Serializable`] trait.
/// The default implementation makes use of [`Serializable`] trait to provide
/// the necessary deserialization functionality without additional code from the
/// consumer.
pub trait DeserializableSlice<const N: usize>: Serializable<N> {
    /// Deserialize a slice of [`u8`] into [`Self`]
    fn from_slice(buf: &[u8]) -> Result<Self, Self::Error>
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

// Auto trait [`DeserializableSlice`] for any type that implements
// [`Serializable`]
impl<T, const N: usize> DeserializableSlice<N> for T where T: Serializable<N> {}
