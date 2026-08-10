// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

mod common;
use common::{Beef, BeefError};
use dusk_bytes::ParseHexStr;

#[test]
fn parse_correct_chars() -> Result<(), BeefError> {
    let beef = Beef::from_hex_str("beef")?;

    assert_eq!(format!("{:x}", beef), "beef");

    Ok(())
}

#[test]
fn parse_invalid_chars() {
    let beef = Beef::from_hex_str("beqf");

    let result = matches!(beef, Err(BeefError::CharNotValid('q', 2)));
    assert!(
        result,
        "Expected parse failing at index 2 for character 'q'"
    )
}

#[test]
fn parse_wrong_chars() {
    let beef = Beef::from_hex_str("abcd");

    let result = matches!(beef, Err(BeefError::InvalidBytes));
    assert!(
        result,
        "Expected parse failing because invalid bytes for Beef"
    )
}

mod constant {
    use dusk_bytes::hex;

    const BEEF: [u8; 2] = hex(b"beef");
    const BEEF_SMALL: [u8; 1] = hex(b"beef");
    const BEEF_BIG: [u8; 3] = hex(b"beef");
    const BEEF_EMPTY: [u8; 0] = hex(b"beef");
    const TRUNCATED_INVALID_SUFFIX: [u8; 1] = hex(b"00gg");
    const EMPTY: [u8; 0] = hex(b"");
    const EMPTY_BIG: [u8; 2] = hex(b"");

    #[test]
    fn const_parse_correct_chars() {
        assert_eq!(BEEF, [0xbe, 0xef]);
        assert_eq!(BEEF_SMALL, [0xbe]);
        assert_eq!(BEEF_BIG, [0xbe, 0xef, 0x0]);
        assert_eq!(BEEF_EMPTY, []);
        assert_eq!(TRUNCATED_INVALID_SUFFIX, [0x0]);
        assert_eq!(EMPTY, []);
        assert_eq!(EMPTY_BIG, [0x0, 0x0]);
    }

    #[test]
    #[should_panic(expected = "hex(): input length must be even")]
    fn reject_odd_length() {
        let _: [u8; 1] = hex(b"f");
    }

    #[test]
    #[should_panic(expected = "hex(): input length must be even")]
    fn reject_odd_length_with_zero_size_destination() {
        let _: [u8; 0] = hex(b"f");
    }

    #[test]
    #[should_panic(expected = "hex(): input length must be even")]
    fn reject_truncated_odd_length() {
        let _: [u8; 1] = hex(b"abc");
    }

    #[test]
    #[should_panic(expected = "hex(): failed to parse the input as hex number")]
    fn reject_invalid_char() {
        let _: [u8; 1] = hex(b"0g");
    }
}
