# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.1] - 2024-01-21
### Fixed
- Fixed a typo in the error message when a value is below the minimum limit.

## [1.1.0] - 2024-01-21
### Added
- Added support for capital 'K' for kilo.
- Added support for underscore separators in numbers.

## [1.0.2] - 2022-10-08
### Fixed
- Fixed panics when parsing SI numbers with multi-byte UTF-8 sequences.

## [1.0.1] - 2022-10-06
### Changed
- Updated documentation for clap v4 API changes.

## [1.0.0] - 2021-12-31
### Fixed
- Fixed typos in documentation.

### Changed
- Changed edition from 2018 to 2021.
- Updated examples to use clap 3.0.0.

## [0.2.0] - 2020-10-18
### Added
- Added `maybe_hex` and `maybe_hex_range` functions.
- Added a changelog.

[Unreleased]: https://github.com/newAM/clap-num/compare/1.1.1...HEAD
[1.1.1]: https://github.com/newAM/clap-num/compare/1.1.0...1.1.1
[1.1.0]: https://github.com/newAM/clap-num/compare/1.0.2...1.1.0
[1.0.2]: https://github.com/newAM/clap-num/compare/1.0.1...1.0.2
[1.0.1]: https://github.com/newAM/clap-num/compare/1.0.0...1.0.1
[1.0.0]: https://github.com/newAM/clap-num/compare/0.2.0...1.0.0
[0.2.0]: https://github.com/newAM/clap-num/releases/tag/0.2.0
