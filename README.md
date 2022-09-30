[![crates.io](https://img.shields.io/crates/v/clap-num.svg)](https://crates.io/crates/clap-num)
[![docs.rs](https://docs.rs/clap-num/badge.svg)](https://docs.rs/clap-num/)
[![Build Status](https://github.com/newAM/clap-num/workflows/CI/badge.svg)](https://github.com/newAM/clap-num/actions)

# clap-num

clap number parsers.

This crate contains functions to validate and parse numerical values from
strings provided by [clap].

## Example

This example allow values for `--frequency` between 800 Hz and 3.333 MHz,
with SI symbols.

```rust
use clap::Parser;
use clap_num::si_number_range;

fn parse_frequency(s: &str) -> Result<u32, String> {
    si_number_range(s, 800, 3_333_000)
}

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, value_parser=parse_frequency)]
    frequency: Option<u32>,
}

let args = Args::parse();
println!("{:?}", args);
```

[clap]: https://github.com/clap-rs/clap
