# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.3] - 2026-08-25

### Changed

- Simplify generated formatting and remove the unused `proc-macro2` dependency
  [#55]
- Update Syn to 3 [#53]
- Raise the MSRV to Rust 1.96.1 [#51]

### Fixed

- Support generic types in the `Hex` and `HexDebug` derives [#44]

### Security

- Document that `Hex` and `HexDebug` expose complete serialized values [#44]
- Forbid unsafe code in `derive-hex` [#45]

## [0.1.2] - 2021-07-15

### Fixed

- Fix incorrect alternate hex display for debug mode [#10]

## [0.1.1] - 2021-01-22

### Added

- Add LICENSE
- Add tests

### Changed

- Change to workspace

## [0.1.0] - 2021-01-13

### Added

- Add `Hex` proc macro
- Add `HexDebug` proc macro

[#10]: https://github.com/dusk-network/dusk-bytes/issues/10
[#44]: https://github.com/dusk-network/dusk-bytes/issues/44
[#45]: https://github.com/dusk-network/dusk-bytes/issues/45
[#51]: https://github.com/dusk-network/dusk-bytes/issues/51
[#53]: https://github.com/dusk-network/dusk-bytes/issues/53
[#55]: https://github.com/dusk-network/dusk-bytes/issues/55
[unreleased]: https://github.com/dusk-network/dusk-bytes/compare/derive-hex-0.1.3...HEAD
[0.1.3]: https://github.com/dusk-network/dusk-bytes/compare/derive-hex-0.1.2...derive-hex-0.1.3
[0.1.2]: https://github.com/dusk-network/dusk-bytes/releases/tag/derive-hex-0.1.2
[0.1.1]: https://github.com/dusk-network/dusk-bytes/releases/tag/derive-hex-0.1.1
[0.1.0]: https://github.com/dusk-network/dusk-bytes/releases/tag/derive-hex-0.1.0
