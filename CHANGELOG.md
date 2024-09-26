# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [4.0.0] - XX Sept 2024

### Changed
- __Breaking Change__: New public API based on `embedded-hal-async`.

## [3.0.0] - 15 Sept 2024 - Yanked

### Changed
- __Breaking Change__: Underlying driver is protected from the public API.

## [2.0.0] - 14 Sept 2024 - Yanked

### Changed
- __Breaking Change__: `RTClock::new()` accepts a shared reference to the shared bus; this allows communicating with multiple rtc chips at different addresses.

## [1.0.0] - 14 Sept 2024 - Yanked

### Changed
- __Breaking Change__: Removed alloc as a default requirement
- __Breaking Change__: Brand new public API via `RTCClock`, refer to docs for details.
- Tested (compilation/clippy) with Embassy and `defmt`.
- TODO: Testing on actual hardware.

## [0.4.4] - 1 March 2024 - Yanked

### Changed
- __Breaking Change__: Fully revised module organisation
- __Breaking Change__: Revised error API
- Renamed `Rv8803<_>::from_i2c0` to `Rv8803<_>::from_i2c`.

## [0.1.0] - 1 January 2022 - Yanked
- Initial release


