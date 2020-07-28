pub use clap;
use core::str::FromStr;

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
/// use clap_validators::validate_integer;
///
/// fn less_than_100(s: &str) -> Result<u8, String> {
///     validate_integer(s, 0, 99)
/// }
///
/// #[derive(Clap, Debug)]
/// struct Change {
///     #[clap(long, parse(try_from_str=less_than_100))]
///     cents: u8,
/// }
/// #
/// # fn main() {
/// #   let args = Change::parse_from(&["", "--cents", "99"]);
/// #   assert_eq!(args.cents, 99);
/// # }
/// ```
///
/// ## Error Messages
///
/// Values exceeding this range will show an error message similar to this:
///
/// ```bash
/// error: Invalid value for '--cents <cents>': exceeds maximum of 99
/// ```
///
/// Values that are not numbers will show an error message similar to this:
///
/// ```bash
/// error: Invalid value for '--cents <cents>': invalid digit found in string
/// ```
///
/// Values result in integer overflow will show an error message like this:
///
/// ```bash
/// error: Invalid value for '--cents <cents>': number too large to fit in target type
/// ```
pub fn validate_integer<T: Ord + PartialOrd + std::fmt::Display>(
    s: &str,
    min: T,
    max: T,
) -> Result<T, String>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    debug_assert!(min <= max, "minimum of {} exceeds maximum of {}", min, max);
    match s.parse::<T>() {
        Ok(v) => {
            if v > max {
                Err(format!("exceeds maximum of {}", max))
            } else if v < min {
                Err(format!("exceeds minimum of {}", min))
            } else {
                Ok(v)
            }
        }
        Err(e) => Err(format!("{}", e)),
    }
}
