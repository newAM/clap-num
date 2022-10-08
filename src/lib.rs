//! clap number parsers.
//!
//! This crate contains functions to validate and parse numerical values from
//! strings provided by [clap].
//!
//! * `maybe_hex`
//!   Validates an unsigned integer value that can be base-10 or base-16.
//! * `maybe_hex_range`
//!   Validates an unsigned integer value that can be base-10 or base-16 within a range.
//! * `number_range`
//!   Validate a signed or unsigned integer value.
//! * `si_number`
//!   Validate a signed or unsigned integer value with a metric prefix.
//! * `si_number_range`
//!   Validate a signed or unsigned integer value with a metric prefix within a range.
//!
//! [clap]: https://github.com/clap-rs/clap
#![deny(missing_docs)]

use core::str::FromStr;
use num_traits::identities::Zero;
use num_traits::{sign, CheckedAdd, CheckedMul, CheckedSub, Num};

fn check_range<T: Ord + std::fmt::Display>(val: T, min: T, max: T) -> Result<T, String>
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
/// use clap::Parser;
/// use clap_num::number_range;
///
/// fn less_than_100(s: &str) -> Result<u8, String> {
///     number_range(s, 0, 99)
/// }
///
/// #[derive(Parser)]
/// struct Change {
///     #[clap(long, value_parser=less_than_100)]
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
/// Values resulting in integer overflow will show an error message similar to this:
///
/// ```text
/// error: Invalid value for '--cents <cents>': number too large to fit in target type
/// ```
///
/// Values exceeding the limits will show an error message similar to this:
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

// helper for mapping errors to strings
fn stringify<T: std::fmt::Display>(e: T) -> String {
    format!("{}", e)
}

#[derive(Copy, Clone)]
enum SiPrefix {
    Yotta,
    Zetta,
    Exa,
    Peta,
    Tera,
    Giga,
    Mega,
    Kilo,
}

impl From<SiPrefix> for char {
    fn from(p: SiPrefix) -> Self {
        match p {
            SiPrefix::Yotta => 'Y',
            SiPrefix::Zetta => 'Z',
            SiPrefix::Exa => 'E',
            SiPrefix::Peta => 'P',
            SiPrefix::Tera => 'T',
            SiPrefix::Giga => 'G',
            SiPrefix::Mega => 'M',
            SiPrefix::Kilo => 'k',
        }
    }
}

impl SiPrefix {
    fn from_char(symbol: char) -> Option<Self> {
        match symbol {
            'Y' => Some(Self::Yotta),
            'Z' => Some(Self::Zetta),
            'E' => Some(Self::Exa),
            'P' => Some(Self::Peta),
            'T' => Some(Self::Tera),
            'G' => Some(Self::Giga),
            'M' => Some(Self::Mega),
            'k' => Some(Self::Kilo),
            _ => None,
        }
    }

    fn multiplier(&self) -> u128 {
        match self {
            SiPrefix::Yotta => 1_000_000_000_000_000_000_000_000,
            SiPrefix::Zetta => 1_000_000_000_000_000_000_000,
            SiPrefix::Exa => 1_000_000_000_000_000_000,
            SiPrefix::Peta => 1_000_000_000_000_000,
            SiPrefix::Tera => 1_000_000_000_000,
            SiPrefix::Giga => 1_000_000_000,
            SiPrefix::Mega => 1_000_000,
            SiPrefix::Kilo => 1_000,
        }
    }

    fn digits(&self) -> usize {
        match self {
            SiPrefix::Yotta => 24,
            SiPrefix::Zetta => 21,
            SiPrefix::Exa => 18,
            SiPrefix::Peta => 15,
            SiPrefix::Tera => 12,
            SiPrefix::Giga => 9,
            SiPrefix::Mega => 6,
            SiPrefix::Kilo => 3,
        }
    }
}

fn parse_post<T>(mut post: String, digits: usize) -> Result<T, String>
where
    <T as std::str::FromStr>::Err: std::fmt::Display,
    T: std::cmp::PartialOrd + std::str::FromStr,
{
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
/// use clap::Parser;
/// use clap_num::si_number;
///
/// #[derive(Parser)]
/// struct Args {
///     #[clap(short, long, value_parser=si_number::<u128>)]
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
    // contains SI symbol
    if let Some(si_prefix) = s.chars().find_map(SiPrefix::from_char) {
        let multiplier: T = T::try_from(si_prefix.multiplier()).map_err(|_| OVERFLOW_MSG)?;

        let (pre_si, post_si) = s.split_once(char::from(si_prefix)).unwrap();

        if pre_si.is_empty() {
            return Err("no value found before SI symbol".to_string());
        }

        // in the format of "1k234" for 1_234
        let (pre, post) = if !post_si.is_empty() {
            (
                pre_si.parse::<T>().map_err(stringify)?,
                parse_post(post_si.to_string(), si_prefix.digits())?,
            )

        // in the format of "1.234k" for 1_234
        } else if let Some((pre_dec, post_dec)) = s.split_once('.') {
            let mut post_dec: String = post_dec.to_string();
            post_dec.pop(); // remove SI symbol
            let post_dec = parse_post(post_dec, si_prefix.digits())?;
            (pre_dec.parse::<T>().map_err(stringify)?, post_dec)

        // no decimal
        } else {
            (pre_si.parse::<T>().map_err(stringify)?, T::zero())
        };

        let pre = pre.checked_mul(&multiplier).ok_or(OVERFLOW_MSG)?;

        if pre >= T::zero() {
            pre.checked_add(&post)
        } else {
            pre.checked_sub(&post)
        }
        .ok_or_else(|| OVERFLOW_MSG.to_string())
    } else {
        // no SI symbol, parse normally
        s.parse::<T>().map_err(stringify)
    }
}

/// Validate a signed or unsigned integer value with a [metric prefix] within
/// a range.
///
/// This combines [`si_number`] and [`number_range`], see the
/// documentation for those functions for details.
///
/// # Example
///
/// This extends the example in [`si_number`], and only allows a range of
/// resistances from 1k to 999.999k.
///
/// ```
/// use clap::Parser;
/// use clap_num::si_number_range;
///
/// fn kilo(s: &str) -> Result<u32, String> {
///     si_number_range(s, 1_000, 999_999)
/// }
///
/// #[derive(Parser)]
/// struct Args {
///     #[clap(short, long, value_parser=kilo)]
///     resistance: u32,
/// }
/// # let args = Args::parse_from(&["", "--resistance", "999k999"]);
/// # assert_eq!(args.resistance, 999_999);
/// ```
///
/// [metric prefix]: https://en.wikipedia.org/wiki/Metric_prefix
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

/// Validates an unsigned integer value that can be base-10 or base-16.
///
/// The number is assumed to be base-10 by default, it is parsed as hex if the
/// number is prefixed with `0x`, case insensitive.
///
/// # Example
///
/// This allows base-10 addresses to be passed normally, or base-16 values to
/// be passed when prefixed with `0x`.
///
/// ```
/// use clap::Parser;
/// use clap_num::maybe_hex;
///
/// #[derive(Parser)]
/// struct Args {
///     #[clap(short, long, value_parser=maybe_hex::<u32>)]
///     address: u32,
/// }
/// # let args = Args::parse_from(&["", "-a", "0x10"]);
/// # assert_eq!(args.address, 16);
/// ```
pub fn maybe_hex<T: Num + sign::Unsigned>(s: &str) -> Result<T, String>
where
    <T as num_traits::Num>::FromStrRadixErr: std::fmt::Display,
{
    const HEX_PREFIX: &str = "0x";
    const HEX_PREFIX_LEN: usize = HEX_PREFIX.len();

    let result = if s.to_ascii_lowercase().starts_with(HEX_PREFIX) {
        T::from_str_radix(&s[HEX_PREFIX_LEN..], 16)
    } else {
        T::from_str_radix(s, 10)
    };

    match result {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("{}", e)),
    }
}

/// Validates an unsigned integer value that can be base-10 or base-16 within
/// a range.
///
/// This combines [`maybe_hex`] and [`number_range`], see the
/// documentation for those functions for details.
///
/// # Example
///
/// This extends the example in [`maybe_hex`], and only allows a range of
/// addresses from `0x100` to `0x200`.
///
/// ```
/// use clap::Parser;
/// use clap_num::maybe_hex_range;
///
/// fn address_in_range(s: &str) -> Result<u32, String> {
///     maybe_hex_range(s, 0x100, 0x200)
/// }
///
/// #[derive(Parser)]
/// struct Args {
///     #[clap(short, long, value_parser=address_in_range)]
///     address: u32,
/// }
/// # let args = Args::parse_from(&["", "-a", "300"]);
/// # assert_eq!(args.address, 300);
/// ```
pub fn maybe_hex_range<T: Num + sign::Unsigned>(s: &str, min: T, max: T) -> Result<T, String>
where
    <T as num_traits::Num>::FromStrRadixErr: std::fmt::Display,
    <T as std::str::FromStr>::Err: std::fmt::Display,
    T: FromStr,
    T: std::fmt::Display,
    T: std::cmp::Ord,
{
    let val = maybe_hex(s)?;
    check_range(val, min, max)
}
