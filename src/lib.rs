//! clap V3 number parsers.
//!
//! This crate contains functions to validate and parse numerical values from
//! strings provided by [clap v3].
//!
//! # Example
//!
//! This example allow values for `--frequency` between 800 Hz and 3.333 MHz,
//! with SI symbols.
//!
//! ```
//! use clap::Clap;
//! use clap_num::si_number_range;
//!
//! fn frequency(s: &str) -> Result<u32, String> {
//!     si_number_range(s, 800, 3_333_000)
//! }
//!
//! #[derive(Clap, Debug)]
//! struct Args {
//!     #[clap(short, long, parse(try_from_str=frequency))]
//!     frequency: Option<u32>,
//! }
//!
//! let args = Args::parse();
//! println!("{:?}", args);
//! ```
//!
//! [clap v3]: https://github.com/clap-rs/clap
#![doc(html_root_url = "https://docs.rs/clap-num/0.1.1")]
#![deny(warnings, missing_docs)]

use core::convert::TryFrom;
use core::str::FromStr;
use num_traits::identities::Zero;
use num_traits::{CheckedAdd, CheckedMul, CheckedSub};

fn check_range<T: Ord + PartialOrd + std::fmt::Display>(val: T, min: T, max: T) -> Result<T, String>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    if val > max {
        Err(format!("exceeds maximum of {}", max))
    } else if val < min {
        Err(format!("exceeds minimum of {}", min))
    } else {
        Ok(val)
    }
}

/// Validate a signed or unsigned integer value.
///
/// # Arguments
///
/// * `s` - String to parse.
/// * `min` - Minimum value, inclusive.
/// * `max` - Maximum value, inclusive.
///
/// # Example
///
/// This allows for a number of cents to be passed in the range of 0-99
/// (inclusive).
///
/// ```
/// use clap::Clap;
/// use clap_num::number_range;
///
/// fn less_than_100(s: &str) -> Result<u8, String> {
///     number_range(s, 0, 99)
/// }
///
/// #[derive(Clap)]
/// struct Change {
///     #[clap(long, parse(try_from_str=less_than_100))]
///     cents: u8,
/// }
/// # let args = Change::parse_from(&["", "--cents", "99"]);
/// # assert_eq!(args.cents, 99);
/// ```
///
/// To run this example run `cargo run --example change`, giving arguments to
/// the program after `--`, for example:
///
/// ```text
/// $ cargo run --example change -- --cents 99
/// Change: 99 cents
/// ```
///
/// ## Error Messages
///
/// Values that are not numbers will show an error message similar to this:
///
/// ```text
/// error: Invalid value for '--cents <cents>': invalid digit found in string
/// ```
///
/// Values result in integer overflow will show an error message similar to this:
///
/// ```text
/// error: Invalid value for '--cents <cents>': number too large to fit in target type
/// ```
///
/// Values that are exceed the limits will show an error message similar to this:
///
/// ```text
/// error: Invalid value for '--cents <cents>': exceeds maximum of 99
/// ```
pub fn number_range<T: Ord + PartialOrd + std::fmt::Display>(
    s: &str,
    min: T,
    max: T,
) -> Result<T, String>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    debug_assert!(min <= max, "minimum of {} exceeds maximum of {}", min, max);
    let val = s.parse::<T>().map_err(stringify)?;
    check_range(val, min, max)
}

static OVERFLOW_MSG: &str = "number too large to fit in target type";

fn stringify<T: std::fmt::Display>(e: T) -> String {
    format!("{}", e)
}

fn overflow_err<T>(_: T) -> String {
    OVERFLOW_MSG.to_string()
}

fn find_si_symbol(s: &str) -> (u128, usize, Option<usize>) {
    for (i, c) in s.chars().enumerate() {
        match c {
            'Y' => return (1_000_000_000_000_000_000_000_000, 24, Some(i)),
            'Z' => return (1_000_000_000_000_000_000_000, 21, Some(i)),
            'E' => return (1_000_000_000_000_000_000, 18, Some(i)),
            'P' => return (1_000_000_000_000_000, 15, Some(i)),
            'T' => return (1_000_000_000_000, 12, Some(i)),
            'G' => return (1_000_000_000, 9, Some(i)),
            'M' => return (1_000_000, 6, Some(i)),
            'k' => return (1_000, 3, Some(i)),
            _ => continue,
        };
    }
    (1, 0, None)
}

fn find_decimal(s: &str) -> Option<usize> {
    for (i, c) in s.chars().enumerate() {
        if c == '.' {
            return Some(i);
        }
    }
    None
}

fn parse_post<T>(post: &str, digits: usize, decimal: bool) -> Result<T, String>
where
    <T as std::str::FromStr>::Err: std::fmt::Display,
    T: std::cmp::PartialOrd + std::str::FromStr,
{
    // strip the leading Si character or decimal
    let mut post = post[1..].to_string();
    // pop the Si character if a decimal value
    if decimal {
        post.pop();
    }
    if post.len() > digits {
        Err(String::from("not an integer"))
    } else {
        while post.len() < digits {
            post.push('0');
        }
        post.parse::<T>().map_err(stringify)
    }
}

/// Validate a signed or unsigned integer value with a [metric prefix].
///
/// This can accept strings with the (case sensitive) SI symbols.
///
/// | Symbol | Name  | Value                             |
/// |--------|-------|-----------------------------------|
/// | Y      | yotta | 1_000_000_000_000_000_000_000_000 |
/// | Z      | zetta | 1_000_000_000_000_000_000_000     |
/// | E      | exa   | 1_000_000_000_000_000_000         |
/// | P      | peta  | 1_000_000_000_000_000             |
/// | T      | tera  | 1_000_000_000_000                 |
/// | G      | giga  | 1_000_000_000                     |
/// | M      | mega  | 1_000_000                         |
/// | k      | kilo  | 1_000                             |
///
/// The strings can be provided with a decimal, or using the SI symbol as the
/// decimal separator.
///
/// | String | Value     |
/// |--------|-----------|
/// | 3k3    | 3300      |
/// | 3.3k   | 3300      |
/// | 1M     | 1_000_000 |
///
/// # Example
///
/// This allows for resistance value to be passed using SI symbols.
///
/// ```
/// use clap::Clap;
/// use clap_num::si_number;
///
/// #[derive(Clap)]
/// struct Args {
///     #[clap(short, long, parse(try_from_str=si_number))]
///     resistance: u128,
/// }
/// # let args = Args::parse_from(&["", "--resistance", "1M1"]);
/// # assert_eq!(args.resistance, 1_100_000);
/// ```
///
/// To run this example use `cargo run --example resistance`, giving arguments
/// to the program after `--`, for example:
///
/// ```text
/// $ cargo run --example resistance -- --resistance 1M1
/// Resistance: 1100000 ohms
/// ```
///
/// [metric prefix]: https://en.wikipedia.org/wiki/Metric_prefix
pub fn si_number<T>(s: &str) -> Result<T, String>
where
    <T as std::convert::TryFrom<u128>>::Error: std::fmt::Display,
    <T as std::str::FromStr>::Err: std::fmt::Display,
    T: CheckedAdd,
    T: CheckedMul,
    T: CheckedSub,
    T: FromStr,
    T: std::cmp::PartialOrd,
    T: TryFrom<u128>,
    T: Zero,
{
    let (multiplier, digits, si_index) = find_si_symbol(s);
    let multiplier = T::try_from(multiplier).map_err(overflow_err)?;

    // contains SI symbol
    if let Some(idx) = si_index {
        if idx == 0 {
            return Err("no value found before SI symbol".to_string());
        };
        let (pre_si, post_si) = s.split_at(idx);

        // in the format of "1k234" for 1_234
        let (pre, post) = if post_si.len() > 1 {
            (
                pre_si.parse::<T>().map_err(stringify)?,
                parse_post(&post_si, digits, false)?,
            )
        // in the format of "1.234k" for 1_234
        } else if let Some(idx) = find_decimal(pre_si) {
            let (pre_dec, post_dec) = s.split_at(idx);
            let post_dec = parse_post(&post_dec, digits, true)?;
            (pre_dec.parse::<T>().map_err(stringify)?, post_dec)
        // no decimal value
        } else {
            (pre_si.parse::<T>().map_err(stringify)?, T::zero())
        };

        let pre = pre
            .checked_mul(&multiplier)
            .ok_or_else(|| OVERFLOW_MSG.to_string())?;

        if pre >= T::zero() {
            pre.checked_add(&post)
                .ok_or_else(|| OVERFLOW_MSG.to_string())
        } else {
            pre.checked_sub(&post)
                .ok_or_else(|| OVERFLOW_MSG.to_string())
        }
    } else {
        // no SI symbol, parse normally
        s.parse::<T>().map_err(stringify)
    }
}

/// Validate a signed or unsigned integer value with a [metric prefix] within
/// a range.
///
/// This effectively combines [`si_number`] and [`number_range`], see the
/// documentation for those functions for details.
///
/// # Example
///
/// This extends the example in [`si_number`], and only allows a range of
/// resistances from 1k to 999.999k.
///
/// ```
/// use clap::Clap;
/// use clap_num::si_number_range;
///
/// fn kilo(s: &str) -> Result<u32, String> {
///     si_number_range(s, 1_000, 999_999)
/// }
///
/// #[derive(Clap)]
/// struct Args {
///     #[clap(short, long, parse(try_from_str=kilo))]
///     resistance: u32,
/// }
/// # let args = Args::parse_from(&["", "--resistance", "999k999"]);
/// # assert_eq!(args.resistance, 999_999);
/// ```
///
/// [metric prefix]: https://en.wikipedia.org/wiki/Metric_prefix
/// [`si_number`]: ./fn.si_number.html
/// [`number_range`]: ./fn.number_range.html
pub fn si_number_range<T: Ord + PartialOrd + std::fmt::Display>(
    s: &str,
    min: T,
    max: T,
) -> Result<T, String>
where
    <T as std::convert::TryFrom<u128>>::Error: std::fmt::Display,
    <T as std::str::FromStr>::Err: std::fmt::Display,
    T: CheckedAdd,
    T: CheckedMul,
    T: CheckedSub,
    T: FromStr,
    T: std::cmp::PartialOrd,
    T: TryFrom<u128>,
    T: Zero,
{
    let val = si_number(s)?;
    check_range(val, min, max)
}
