// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

#![deny(missing_docs)]
#![no_std]
#![doc = include_str!("../README.md")]

mod errors;
mod parse;
mod primitive;
mod serialize;

pub use derive_hex::{Hex, HexDebug};
pub use errors::{BadLength, Error, InvalidChar};
pub use parse::{hex, ParseHexStr};
pub use serialize::{DeserializableSlice, Read, Serializable, Write};
