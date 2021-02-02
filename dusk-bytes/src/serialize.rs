// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use super::errors::{BadLength, Error};

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

    /// Deserialize a mutable slice of [`u8`] into [`Self`].
    /// The bytes read are consumed and removed from the given slice.
    fn from_slice_mut<T>(buf: &mut T) -> Result<Self, Self::Error>
    where
        T: Read,
        Self: Sized,
        Self::Error: BadLength,
    {
        let mut bytes = [0u8; N];
        buf.read(&mut bytes)
            .map_err(|_| Self::Error::bad_length(buf.size(), N))?;

        Self::from_bytes(&bytes)
    }
}

// Auto trait [`DeserializableSlice`] for any type that implements
// [`Serializable`]
impl<T, const N: usize> DeserializableSlice<N> for T where T: Serializable<N> {}

pub trait Read {
    fn size(&self) -> usize;
    fn read(&mut self, buf: &mut [u8]) -> Result<(), Error>;
}

impl Read for &[u8] {
    #[inline]
    fn size(&self) -> usize {
        self.len()
    }

    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        if buf.len() > self.len() {
            return Err(Error::bad_length(self.len(), buf.len()));
        }
        let amt = buf.len();
        let (a, b) = self.split_at(amt);

        // First check if the amount of bytes we want to read is small:
        // `copy_from_slice` will generally expand to a call to `memcpy`, and
        // for a single byte the overhead is significant.
        if amt == 1 {
            buf[0] = a[0];
        } else {
            buf[..amt].copy_from_slice(a);
        }

        *self = b;
        Ok(())
    }
}
