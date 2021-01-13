// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use dusk_bytes::{BadLength, DeserializableSlice, Serializable};

use dusk_bytes::HexDebug;
#[derive(HexDebug)]
struct Beef {}

#[derive(Debug)]
enum BeefError {
    InvalidBytes,
    UnexpectedEof,
}

impl Serializable<2> for Beef {
    type Error = BeefError;
    fn from_bytes(buf: &[u8; Self::SIZE]) -> Result<Self, Self::Error> {
        if buf[0] == 0xbe && buf[1] == 0xef {
            Ok(Self {})
        } else {
            Err(BeefError::InvalidBytes)
        }
    }

    fn to_bytes(&self) -> [u8; Self::SIZE] {
        [0xbe, 0xef]
    }
}

// This implements automatically `from_bytes_slice` and length checks
impl DeserializableSlice<2> for Beef {}

// Implementing DeserializableSlice requires `Error` to implements `BadLength`
// too
impl BadLength for BeefError {
    fn bad_length(_found: usize, _expected: usize) -> Self {
        Self::UnexpectedEof
    }
}

#[test]
fn expected_size() {
    assert_eq!(Beef::SIZE, 2, "Expected Beef's size of 2");
}

mod from_bytes {
    use super::*;

    #[test]
    fn correct_buffer() {
        let buf = [0xbe, 0xef];
        let beef = Beef::from_bytes(&buf);

        assert!(beef.is_ok(), "Structure created without error");
    }

    #[test]
    fn wrong_buffer() {
        let buf = [0x0, 0x1];
        let beef = Beef::from_bytes(&buf);

        let result = matches!(beef, Err(BeefError::InvalidBytes));

        assert!(result, "Invalid representation passed");
    }

    #[test]
    fn buffer_too_small() {
        let beef = Beef::from_slice(&[0x0]);

        let result = matches!(beef, Err(BeefError::UnexpectedEof));

        assert!(result, "Not enough bytes to parse");
    }

    #[test]
    fn bigger_buffer() {
        let beef = Beef::from_slice(&[0xbe, 0xef, 0x10, 0x20]);

        assert!(beef.is_ok(), "Structure created without error");
    }
}

mod to_bytes {
    use super::*;

    #[test]
    fn it_works() {
        let beef = Beef {};

        assert_eq!(beef.to_bytes(), [0xbe, 0xef]);
    }
}

mod functions {
    use super::*;

    fn generic_info<S, const N: usize>(obj: S) -> String
    where
        S: Serializable<N> + std::fmt::Debug,
        S::Error: std::fmt::Debug,
    {
        format!("Size: {:?}, Bytes: {:?}", S::SIZE, obj.to_bytes())
    }

    #[test]
    fn accept_generic_serializable() {
        let beef = Beef {};
        let info = generic_info(beef);

        // it's also possible to enforce the size for the compiler,
        // so:

        //     let info = generic_info::<_, 1>(beef);

        // will prevent the compiler to proceed since the size of
        // beef is `2`.

        assert_eq!(info, "Size: 2, Bytes: [190, 239]");
    }

    #[test]
    fn formatting() {
        let beef = Beef {};

        assert_eq!(format!("{:x}", beef), "beef");
        assert_eq!(format!("{:#x}", beef), "0xbeef");
        assert_eq!(format!("{:X}", beef), "BEEF");
        assert_eq!(format!("{:#X}", beef), "0xBEEF");
    }
}
