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

    /// Deserialize the type reading the bytes from a reader.
    /// The bytes read are removed from the reader.
    fn from_reader<R>(buf: &mut R) -> Result<Self, Self::Error>
    where
        R: Read,
        Self: Sized,
        Self::Error: BadLength,
    {
        let mut bytes = [0u8; N];
        buf.read(&mut bytes)
            .map_err(|_| Self::Error::bad_length(buf.capacity(), N))?;

        Self::from_bytes(&bytes)
    }
}

// Auto trait [`DeserializableSlice`] for any type that implements
// [`Serializable`]
impl<T, const N: usize> DeserializableSlice<N> for T where T: Serializable<N> {}

// The `Read` trait allows for reading bytes from a source.
///
/// Implementors of the `Read` trait are called 'readers'.
///
/// Readers are defined by one required method, [`read()`]. Each call to
/// [`read()`] will attempt to pull bytes from this source into a provided
/// buffer.
pub trait Read {
    /// Returns the number of elements the Reader can hold.
    fn capacity(&self) -> usize;

    /// Pull some bytes from this source into the specified buffer, returning
    /// how many bytes were read.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error>;
}

impl Read for &[u8] {
    #[inline]
    fn capacity(&self) -> usize {
        self.len()
    }

    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
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
        Ok(amt)
    }
}

// A trait for objects which are byte-oriented sinks.
///
/// Implementors of the `Write` trait are sometimes called 'writers'.
///
/// Writers are defined by one required method, [`write()`].
pub trait Write {
    /// Write a buffer into this writer, returning how many bytes were written.
    ///
    /// This function will attempt to write the entire contents of `buf`, but
    /// the entire write may not succeed, or the write may also generate an
    /// error.
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
}

impl Write for &mut [u8] {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        if buf.len() > self.len() {
            return Err(Error::bad_length(self.len(), buf.len()));
        }
        let amt = buf.len();

        let (a, b) = core::mem::replace(self, &mut []).split_at_mut(amt);
        a.copy_from_slice(&buf[..amt]);
        *self = b;
        Ok(amt)
    }
}
