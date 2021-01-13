// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

#![feature(external_doc)]
#![deny(missing_docs)]
#![doc(include = "../README.md")]
#![no_std]

mod errors;
mod parse;
mod serialize;

pub use errors::{BadLength, Error, InvalidChar};
pub use parse::ParseHexStr;
pub use serialize::{DeserializableSlice, Serializable};

pub use derive_macro::{Hex, HexDebug};
