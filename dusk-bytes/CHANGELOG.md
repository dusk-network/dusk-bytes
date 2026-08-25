# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Add a default-on `derive` feature and allow dependency-free builds with
  default features disabled [#42]

### Changed

- Borrow the checked input prefix in `DeserializableSlice::from_slice` instead
  of copying it into a temporary array [#46]
- Raise the MSRV to Rust 1.96.1 [#51]

### Fixed

- Make `DeserializableSlice::from_reader` fill its complete destination and
  reject invalid read counts or early EOF [#38]
- Reject odd-length input in the const `hex` parser before indexing [#39]

### Security

- Forbid unsafe code and enforce warning-free documentation [#45]

## [0.1.8-rc.0] - 2026-02-03

### Changed

- Set the MSRV to Rust 1.85 and migrate the crate to Rust 2024 edition
- Refresh the crate documentation and package metadata

## [0.1.7] - 2023-11-01

### Added

- Implement little-endian `Serializable` support for `u8` and all signed
  integer primitives

## [0.1.6] - 2022-07-21

### Changed

- Support stable Rust by removing the obsolete `const_panic` feature gate

## [0.1.5] - 2021-07-16

### Added

- Add the const `hex` function for compile-time hexadecimal parsing

### Changed

- Republish the changes from the yanked `0.2.0` release on the intended `0.1.x`
  release line

## [0.2.0] - 2021-07-15 [YANKED]

### Added

- Add the const `hex` function for compile-time hexadecimal parsing

### Changed

- This release used an unintended version and was republished as `0.1.5`

## [0.1.4] - 2021-06-07

### Fixed

- Include the crate README in rustdoc without the obsolete `external_doc`
  feature

## [0.1.3] - 2021-02-04

### Added

- Add public `Read` and `Write` traits with slice implementations

### Changed

- Report reader capacity when `DeserializableSlice::from_reader` cannot fill
  its destination

## [0.1.2] - 2021-02-03

### Added

- Add `DeserializableSlice::from_reader` and the reader abstraction it uses
- Implement little-endian `Serializable` support for `u16`, `u32`, `u64`, and
  `u128`

## [0.1.1] - 2021-01-22

### Added

- Blanket-implement `DeserializableSlice` and `ParseHexStr` for every
  `Serializable` type

## [0.1.0] - 2021-01-20

### Added

- Initial `no_std` release with fixed-size serialization, slice
  deserialization, hexadecimal parsing, error traits, and hex-formatting derive
  re-exports

[#38]: https://github.com/dusk-network/dusk-bytes/issues/38
[#39]: https://github.com/dusk-network/dusk-bytes/issues/39
[#42]: https://github.com/dusk-network/dusk-bytes/issues/42
[#45]: https://github.com/dusk-network/dusk-bytes/issues/45
[#46]: https://github.com/dusk-network/dusk-bytes/issues/46
[#51]: https://github.com/dusk-network/dusk-bytes/issues/51
[unreleased]: https://github.com/dusk-network/dusk-bytes/compare/dusk-bytes-0.1.8-rc.0...HEAD
[0.1.8-rc.0]: https://github.com/dusk-network/dusk-bytes/compare/dusk-bytes-0.1.7...dusk-bytes-0.1.8-rc.0
[0.1.7]: https://github.com/dusk-network/dusk-bytes/compare/dusk-bytes-0.1.6...dusk-bytes-0.1.7
[0.1.6]: https://github.com/dusk-network/dusk-bytes/compare/41dd9bbee37769c5fa1fa3079ee61cb6887c1bd3...dusk-bytes-0.1.6
[0.1.5]: https://github.com/dusk-network/dusk-bytes/compare/dusk-bytes-0.1.4...41dd9bbee37769c5fa1fa3079ee61cb6887c1bd3
[0.2.0]: https://github.com/dusk-network/dusk-bytes/compare/dusk-bytes-0.1.4...05b67a5e0066b14d78f4db79e218d27bd71ba25d
[0.1.4]: https://github.com/dusk-network/dusk-bytes/compare/dusk-bytes-0.1.3...dusk-bytes-0.1.4
[0.1.3]: https://github.com/dusk-network/dusk-bytes/compare/dusk-bytes-0.1.2...dusk-bytes-0.1.3
[0.1.2]: https://github.com/dusk-network/dusk-bytes/compare/v0.1.1...dusk-bytes-0.1.2
[0.1.1]: https://github.com/dusk-network/dusk-bytes/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/dusk-network/dusk-bytes/releases/tag/v0.1.0
