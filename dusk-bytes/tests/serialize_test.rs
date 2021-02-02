// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

mod common;
use common::{Beef, BeefError};

use dusk_bytes::{DeserializableSlice, Serializable};

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

    #[test]
    fn mutable_bigger_and_wrong_buffer() {
        let mut bytes = &[0xbe, 0xef, 0x10, 0x20][..];
        let beef = Beef::from_slice_mut(&mut bytes);

        assert!(beef.is_ok(), "Structure created without error");
        assert_eq!(bytes, [0x10, 0x20], "Buffer Consumed");

        let beef = Beef::from_slice_mut(&mut bytes);
        let result = matches!(beef, Err(BeefError::InvalidBytes));

        assert!(result, "Invalid representation passed");
        assert!(bytes.is_empty(), "Buffer Consumed");
    }

    #[test]
    fn mutable_bigger_and_not_enough_buffer() {
        let mut bytes = &[0xbe, 0xef, 0x10][..];
        let beef = Beef::from_slice_mut(&mut bytes);

        assert!(beef.is_ok(), "Structure created without error");
        assert_eq!(bytes, [0x10], "Buffer Consumed");

        let beef = Beef::from_slice_mut(&mut bytes);
        let result = matches!(beef, Err(BeefError::UnexpectedEof));

        assert!(result, "Not enough bytes to parse");
        assert_eq!(bytes, [0x10], "Buffer is not consumed");
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
}
