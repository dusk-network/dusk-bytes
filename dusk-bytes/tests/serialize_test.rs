// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

mod common;
use common::{Beef, BeefError};

use dusk_bytes::{DeserializableSlice, Error, Serializable};
use std::fmt::Debug;

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
        let beef = Beef::from_reader(&mut bytes);

        assert!(beef.is_ok(), "Structure created without error");
        assert_eq!(bytes, [0x10, 0x20], "Buffer Consumed");

        let beef = Beef::from_reader(&mut bytes);
        let result = matches!(beef, Err(BeefError::InvalidBytes));

        assert!(result, "Invalid representation passed");
        assert!(bytes.is_empty(), "Buffer Consumed");
    }

    #[test]
    fn mutable_bigger_and_not_enough_buffer() {
        let mut bytes = &[0xbe, 0xef, 0x10][..];
        let beef = Beef::from_reader(&mut bytes);

        assert!(beef.is_ok(), "Structure created without error");
        assert_eq!(bytes, [0x10], "Buffer Consumed");

        let beef = Beef::from_reader(&mut bytes);
        let result = matches!(beef, Err(BeefError::UnexpectedEof));

        assert!(result, "Not enough bytes to parse");
        assert_eq!(bytes, [0x10], "Buffer is not consumed");
    }

    #[test]
    fn primitive_types() -> Result<(), Error> {
        assert_eq!(0x01_u8, u8::from_bytes(&[0x01])?);
        assert_eq!(0x0102_u16, u16::from_bytes(&[0x02, 0x01])?);
        assert_eq!(0x01020304_u32, u32::from_bytes(&[0x04, 0x03, 0x02, 0x01])?);
        assert_eq!(
            0x0102030405060708_u64,
            u64::from_bytes(&[0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01])?
        );
        assert_eq!(
            0x0102030405060708090a0b0c0d0e0f10_u128,
            u128::from_bytes(&[
                0x10, 0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x07,
                0x06, 0x05, 0x04, 0x03, 0x02, 0x01
            ])?
        );

        Ok(())
    }

    #[test]
    fn primitive_types_from_reader() -> Result<(), Error> {
        let mut buffer = &[
            0x03, 0x02, 0x01, 0x04, 0x03, 0x02, 0x01, 0x08, 0x07, 0x06, 0x05,
            0x04, 0x03, 0x02, 0x01, 0x10, 0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a,
            0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01,
        ][..];

        assert_eq!(0x03_u8, u8::from_reader(&mut buffer)?);
        assert_eq!(0x0102_u16, u16::from_reader(&mut buffer)?);
        assert_eq!(0x01020304_u32, u32::from_reader(&mut buffer)?);
        assert_eq!(0x0102030405060708_u64, u64::from_reader(&mut buffer)?);
        assert_eq!(
            0x0102030405060708090a0b0c0d0e0f10_u128,
            u128::from_reader(&mut buffer)?
        );

        Ok(())
    }

    #[test]
    fn primitive_types_buffer_too_small() {
        assert!(
            matches!(
                u8::from_slice(&[]),
                Err(Error::BadLength {
                    found: 0,
                    expected: 1
                })
            ),
            "Not enough bytes to parse"
        );

        assert!(
            matches!(
                u16::from_slice(&[0x01]),
                Err(Error::BadLength {
                    found: 1,
                    expected: 2
                })
            ),
            "Not enough bytes to parse"
        );

        assert!(
            matches!(
                u32::from_slice(&[0x01, 0x02]),
                Err(Error::BadLength {
                    found: 2,
                    expected: 4
                })
            ),
            "Not enough bytes to parse"
        );

        assert!(
            matches!(
                u64::from_slice(&[]),
                Err(Error::BadLength {
                    found: 0,
                    expected: 8
                })
            ),
            "Not enough bytes to parse"
        );

        assert!(
            matches!(
                u128::from_slice(&[0x01, 0x02, 0x03]),
                Err(Error::BadLength {
                    found: 3,
                    expected: 16
                })
            ),
            "Not enough bytes to parse"
        );

        assert!(
            matches!(
                i8::from_slice(&[]),
                Err(Error::BadLength {
                    found: 0,
                    expected: 1
                })
            ),
            "Not enough bytes to parse i8"
        );

        assert!(
            matches!(
                i16::from_slice(&[0xff]),
                Err(Error::BadLength {
                    found: 1,
                    expected: 2
                })
            ),
            "Not enough bytes to parse i16"
        );

        assert!(
            matches!(
                i32::from_slice(&[0xff, 0xe4]),
                Err(Error::BadLength {
                    found: 2,
                    expected: 4
                })
            ),
            "Not enough bytes to parse i32"
        );

        assert!(
            matches!(
                i64::from_slice(&[0xff, 0xf4]),
                Err(Error::BadLength {
                    found: 2,
                    expected: 8
                })
            ),
            "Not enough bytes to parse i64"
        );

        assert!(
            matches!(
                i128::from_slice(&[0xff, 0xe4, 0xef]),
                Err(Error::BadLength {
                    found: 3,
                    expected: 16
                })
            ),
            "Not enough bytes to parse i128"
        );
    }
}

mod to_bytes {
    use super::*;

    #[test]
    fn it_works() {
        let beef = Beef {};

        assert_eq!(beef.to_bytes(), [0xbe, 0xef]);
    }

    #[test]
    fn primitive_types() {
        assert_eq!(0x01_u8.to_bytes(), [0x01]);
        assert_eq!(0x0102_u16.to_bytes(), [0x02, 0x01]);
        assert_eq!(0x01020304_u32.to_bytes(), [0x04, 0x03, 0x02, 0x01]);
        assert_eq!(
            0x0102030405060708_u64.to_bytes(),
            [0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01]
        );
        assert_eq!(
            0x0102030405060708090a0b0c0d0e0f10_u128.to_bytes(),
            [
                0x10, 0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x07,
                0x06, 0x05, 0x04, 0x03, 0x02, 0x01
            ]
        );
        assert_eq!((-1_i8).to_bytes(), [0xff]);
        assert_eq!(0x0102_i16.to_bytes(), [0x02, 0x01]);
        assert_eq!((-12345678_i32).to_bytes(), [0xb2, 0x9e, 0x43, 0xff]);
        assert_eq!(
            0x0102030405060708_i64.to_bytes(),
            [0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01]
        );
        assert_eq!(
            (-1234567890123456789012345678901234_i128).to_bytes(),
            [
                0x0e, 0x50, 0x69, 0x81, 0x2f, 0xa3, 0x7d, 0x21, 0xcd, 0x68,
                0x00, 0x90, 0x21, 0xc3, 0xff, 0xff,
            ]
        );
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

    fn test_primitive<T, const SIZE: usize>(original: T)
    where
        T: Serializable<SIZE> + Eq + Debug,
        <T as Serializable<SIZE>>::Error: Debug,
    {
        let serialized = <T as Serializable<SIZE>>::to_bytes(&original);
        let deserialized =
            <T as Serializable<SIZE>>::from_bytes(&serialized).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_serializable_deserialize_primitives() {
        test_primitive::<u8, 1>(0x01);
        test_primitive::<u16, 2>(0x0102);
        test_primitive::<u32, 4>(0x01020304);
        test_primitive::<u64, 8>(0x0102030405060708);
        test_primitive::<u128, 16>(0x0102030405060708090a0b0c0d0e0f10);
        test_primitive::<i8, 1>(-1);
        test_primitive::<i16, 2>(-1234);
        test_primitive::<i32, 4>(-12345678);
        test_primitive::<i64, 8>(-1234567890123456);
        test_primitive::<i128, 16>(-1234567890123456789012345678901234);
    }
}

mod buffers {
    use super::*;

    #[test]
    fn write_buffer() -> Result<(), Error> {
        use dusk_bytes::Write;

        let mut buffer = [0u8; Beef::SIZE * 2 + 1];
        let beef = Beef {};

        let mut writer = &mut buffer[..];
        writer.write(&beef.to_bytes())?;
        writer.write(&beef.to_bytes())?;

        assert_eq!(writer.len(), 1, "Writer consumed");
        assert_eq!(&buffer, &[0xbe, 0xef, 0xbe, 0xef, 0x0], "Buffer written");

        Ok(())
    }

    #[test]
    fn source_buffer_too_small() -> Result<(), Error> {
        use dusk_bytes::Write;

        let mut buffer = [0u8; Beef::SIZE + 1];
        let beef = Beef {};

        let mut writer = &mut buffer[..];
        writer.write(&beef.to_bytes())?;

        assert!(
            matches!(
                writer.write(&beef.to_bytes()),
                Err(Error::BadLength {
                    found: 1,
                    expected: 2
                })
            ),
            "Dest buffer too small"
        );

        Ok(())
    }
}
