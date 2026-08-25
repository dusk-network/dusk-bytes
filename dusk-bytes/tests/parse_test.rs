// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

mod common;
use common::{Beef, BeefError};
use dusk_bytes::{Error, ParseHexStr, Serializable};

#[test]
fn parse_correct_chars() -> Result<(), BeefError> {
    let beef = Beef::from_hex_str("beef")?;

    assert_eq!(beef.to_bytes(), [0xbe, 0xef]);

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

#[test]
fn parse_reports_length_and_low_nibble_errors() {
    assert_eq!(
        u16::from_hex_str("bee"),
        Err(Error::BadLength {
            found: 3,
            expected: 4,
        })
    );
    assert_eq!(
        u8::from_hex_str("0g"),
        Err(Error::InvalidChar { ch: 'g', index: 1 })
    );
}

#[test]
fn parse_accepts_uppercase_hex() {
    assert_eq!(u16::from_hex_str("BEEF"), Ok(0xefbe));
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
    fn runtime_matches_const_fixtures() {
        let input = std::hint::black_box(*b"beef");
        let exact: [u8; 2] = hex(&input);
        let truncated: [u8; 1] = hex(&input);
        let padded: [u8; 3] = hex(&input);

        assert_eq!(exact, BEEF);
        assert_eq!(truncated, BEEF_SMALL);
        assert_eq!(padded, BEEF_BIG);
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
