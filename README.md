[![crates.io](https://img.shields.io/crates/v/clap-num.svg)](https://crates.io/crates/clap-num)
[![docs.rs](https://docs.rs/clap-num/badge.svg)](https://docs.rs/clap-num/)
[![Build Status](https://github.com/newAM/clap-num/workflows/CI/badge.svg)](https://github.com/newAM/clap-num/actions)

# clap-num

clap number parsers.

This crate contains functions to validate and parse numerical values from
strings provided by [clap].

* `maybe_hex`
  Validates an unsigned integer value that can be base-10 or base-16.
* `maybe_hex_range`
  Validates an unsigned integer value that can be base-10 or base-16 within a range.
* `number_range`
  Validate a signed or unsigned integer value.
* `si_number`
  Validate a signed or unsigned integer value with a metric prefix.
* `si_number_range`
  Validate a signed or unsigned integer value with a metric prefix within a range.

[clap]: https://github.com/clap-rs/clap
