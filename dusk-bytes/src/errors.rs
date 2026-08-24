// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

/// Trait implemented by error types used by
/// [`DeserializableSlice::from_slice`](crate::DeserializableSlice::from_slice).
/// The method is called when the given slice is shorter than the required size.
pub trait BadLength {
    /// Invoked when a buffer of bad length is given to
    /// [`DeserializableSlice::from_slice`](crate::DeserializableSlice::from_slice).
    fn bad_length(found: usize, expected: usize) -> Self;
}

/// Trait implemented by error types used by
/// [`ParseHexStr::from_hex_str`](crate::ParseHexStr::from_hex_str).
/// The method is called when an invalid character is found in the string slice.
pub trait InvalidChar {
    /// Invoked when a string slice with a non-hex character is given to
    /// [`ParseHexStr::from_hex_str`](crate::ParseHexStr::from_hex_str).
    fn invalid_char(ch: char, index: usize) -> Self;
}

/// Dusk Bytes operation error variants
#[derive(Copy, Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Error {
    /// Generic error that can be returned in a
    /// [`Serializable::from_bytes`](crate::Serializable::from_bytes)
    /// implementation.
    InvalidData,
    /// Automatically returned from the default implementation of
    /// [`DeserializableSlice::from_slice`](crate::DeserializableSlice::from_slice)
    /// if the slice given is smaller than the mandatory size for the struct.
    BadLength {
        /// The slice's length
        found: usize,
        /// The expected slice's length
        expected: usize,
    },
    /// Automatically returned from the default implementation of
    /// [`ParseHexStr::from_hex_str`](crate::ParseHexStr::from_hex_str) if an
    /// invalid character is found in the string slice.
    InvalidChar {
        /// The invalid character found
        ch: char,
        /// The character's index
        index: usize,
    },
}

impl BadLength for Error {
    fn bad_length(found: usize, expected: usize) -> Self {
        Self::BadLength { found, expected }
    }
}

impl InvalidChar for Error {
    fn invalid_char(ch: char, index: usize) -> Self {
        Self::InvalidChar { ch, index }
    }
}
