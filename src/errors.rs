// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

/// Trait to be implemented for the associated  Error used in
/// [`DeserializableSlice::from_slice`]. The function is called if the slice
/// given is smaller than the mandatory size for the struct.
pub trait BadLength {
    /// Invoked when a buffer of bad length is given to [`from_slice`]
    fn bad_length(found: usize, expected: usize) -> Self;
}

/// Trait to be implemented for the associated Error used in
/// [`ParseHexStr::from_hex_str`].
/// The function is called  if an invalid character is found in the string
/// slice.
pub trait InvalidChar {
    /// Invoked when a string slice with a non hex character is is give to
    /// [`ParseHexStr::from_hex_str`]
    fn invalid_char(ch: char, index: usize) -> Self;
}

/// Dusk Bytes operation error variants
#[derive(Copy, Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Error {
    /// Generic error that can be returned in a [`Deserializable::from_bytes`]
    /// implementation
    InvalidData,
    /// Automatically returned from the default implementation of
    /// [`DeserializableSlice::from_slice`] if the slice given is smaller than
    /// the mandatory size for the struct.
    BadLength {
        /// The slice's length
        found: usize,
        /// The expected slice's length
        expected: usize,
    },
    /// Automatically returned from the default implementation of
    /// [`ParseHexStr::from_hex_str`] if an invalid character is found in the
    /// string slice.
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
