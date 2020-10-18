![Maintenance](https://img.shields.io/badge/maintenance-as--is-yellow.svg)
[![crates.io](https://img.shields.io/crates/v/clap-num.svg)](https://crates.io/crates/clap-num)
[![docs.rs](https://docs.rs/clap-num/badge.svg)](https://docs.rs/clap-num/)
[![Build Status](https://github.com/newAM/clap-num/workflows/CI/badge.svg)](https://github.com/newAM/clap-num/actions)

# clap-num

clap V3 number parsers.

This crate contains functions to validate and parse numerical values from
strings provided by [clap v3].

## Example

This example allow values for `--frequency` between 800 Hz and 3.333 MHz,
with SI symbols.

```rust
use clap::Clap;
use clap_num::si_number_range;

fn frequency(s: &str) -> Result<u32, String> {
    si_number_range(s, 800, 3_333_000)
}

#[derive(Clap, Debug)]
struct Args {
    #[clap(short, long, parse(try_from_str=frequency))]
    frequency: Option<u32>,
}

let args = Args::parse();
println!("{:?}", args);
```

[clap v3]: https://github.com/clap-rs/clap
