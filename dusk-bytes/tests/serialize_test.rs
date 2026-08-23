// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

mod common;
use common::{Beef, BeefError};
use dusk_bytes::{DeserializableSlice, Error, Read, Serializable};

#[test]
fn expected_size() {
    assert_eq!(Beef::SIZE, 2, "Expected Beef's size of 2");
}

mod from_bytes {
    use super::*;

    struct ChunkedReader<'a> {
        bytes: &'a [u8],
        chunk_size: usize,
    }

    impl<'a> ChunkedReader<'a> {
        fn new(bytes: &'a [u8], chunk_size: usize) -> Self {
            Self { bytes, chunk_size }
        }
    }

    impl Read for ChunkedReader<'_> {
        fn capacity(&self) -> usize {
            self.bytes.len()
        }

        fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
            let amount = buf.len().min(self.chunk_size).min(self.bytes.len());
            buf[..amount].copy_from_slice(&self.bytes[..amount]);
            self.bytes = &self.bytes[amount..];
            Ok(amount)
        }
    }

    struct InvalidReader;

    impl Read for InvalidReader {
        fn capacity(&self) -> usize {
            0
        }

        fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
            Ok(buf.len() + 1)
        }
    }

    struct FailingReader {
        error: Error,
        read_once: bool,
        remaining: usize,
    }

    impl FailingReader {
        fn new(error: Error, remaining: usize) -> Self {
            Self {
                error,
                read_once: false,
                remaining,
            }
        }
    }

    impl Read for FailingReader {
        fn capacity(&self) -> usize {
            if self.read_once {
                self.remaining
            } else {
                self.remaining + 1
            }
        }

        fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
            if self.read_once {
                Err(self.error)
            } else {
                buf[0] = 0x04;
                self.read_once = true;
                Ok(1)
            }
        }
    }

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
    fn from_reader_fills_buffer_across_short_reads() -> Result<(), Error> {
        let mut reader = ChunkedReader::new(&[0x04, 0x03, 0x02, 0x01, 0xff], 1);

        assert_eq!(0x01020304_u32, u32::from_reader(&mut reader)?);
        assert_eq!(reader.bytes, &[0xff]);

        Ok(())
    }

    #[test]
    fn from_reader_rejects_early_eof() {
        let mut reader = ChunkedReader::new(&[0x04, 0x03], 1);

        assert_eq!(
            u32::from_reader(&mut reader),
            Err(Error::BadLength {
                found: 2,
                expected: 4
            })
        );
        assert!(reader.bytes.is_empty());
    }

    #[test]
    fn from_reader_rejects_invalid_read_count() {
        assert_eq!(
            u32::from_reader(&mut InvalidReader),
            Err(Error::BadLength {
                found: 0,
                expected: 4
            })
        );
    }

    #[test]
    fn from_reader_reports_reader_capacity_on_error() {
        let cases = [
            (Error::InvalidData, 0, 1),
            (
                Error::BadLength {
                    found: 1000,
                    expected: 3,
                },
                2,
                3,
            ),
        ];

        for (error, remaining, found) in cases {
            let mut reader = FailingReader::new(error, remaining);
            assert_eq!(
                u32::from_reader(&mut reader),
                Err(Error::BadLength { found, expected: 4 })
            );
        }
    }

    #[test]
    fn slice_reader_preserves_input_on_early_eof() {
        let input = [0x04, 0x03];
        let mut reader = &input[..];

        assert_eq!(
            u32::from_reader(&mut reader),
            Err(Error::BadLength {
                found: 2,
                expected: 4
            })
        );
        assert_eq!(reader, input);
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
        assert_eq!(
            u8::from_slice(&[]),
            Err(Error::BadLength {
                found: 0,
                expected: 1,
            })
        );
        assert_eq!(
            i128::from_slice(&[0xff, 0xe4, 0xef]),
            Err(Error::BadLength {
                found: 3,
                expected: 16,
            })
        );
    }
}

mod primitive_types {
    use super::*;

    macro_rules! assert_serialization {
        ($ty:ty, $value:expr, $bytes:expr) => {{
            let value: $ty = $value;
            let bytes = $bytes;

            assert_eq!(value.to_bytes(), bytes);
            assert_eq!(<$ty>::from_bytes(&bytes), Ok(value));
        }};
    }

    #[test]
    fn golden_serialization() {
        assert_serialization!(u8, 0x01, [0x01]);
        assert_serialization!(u16, 0x0102, [0x02, 0x01]);
        assert_serialization!(u32, 0x01020304, [0x04, 0x03, 0x02, 0x01]);
        assert_serialization!(
            u64,
            0x0102030405060708,
            [0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01]
        );
        assert_serialization!(
            u128,
            0x0102030405060708090a0b0c0d0e0f10,
            [
                0x10, 0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x07,
                0x06, 0x05, 0x04, 0x03, 0x02, 0x01,
            ]
        );
        assert_serialization!(i8, -1, [0xff]);
        assert_serialization!(i16, 0x0102, [0x02, 0x01]);
        assert_serialization!(i32, -12345678, [0xb2, 0x9e, 0x43, 0xff]);
        assert_serialization!(
            i64,
            0x0102030405060708,
            [0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01]
        );
        assert_serialization!(
            i128,
            -1234567890123456789012345678901234,
            [
                0x0e, 0x50, 0x69, 0x81, 0x2f, 0xa3, 0x7d, 0x21, 0xcd, 0x68,
                0x00, 0x90, 0x21, 0xc3, 0xff, 0xff,
            ]
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
