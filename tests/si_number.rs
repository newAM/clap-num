use clap::Parser;
use clap_num::si_number;

#[test]
fn utf8_byte_index_not_char_boundry() {
    let _ = si_number::<u64>("˲TP");
}

#[test]
fn utf8_byte_index_not_char_boundry_with_decimal() {
    let _ = si_number::<u64>("˲.E");
}

// standalone basic tests
#[cfg(test)]
mod basic {
    use super::*;

    macro_rules! pos {
        ($NAME:ident, $VAL:expr, $RESULT:expr) => {
            #[test]
            fn $NAME() {
                assert_eq!(si_number($VAL), Ok($RESULT));
            }
        };
    }

    macro_rules! neg {
        ($NAME:ident, $VAL:expr, $TYPE:ident, $RESULT:expr) => {
            #[test]
            fn $NAME() {
                let num: Result<$TYPE, String> = si_number($VAL);
                assert_eq!(num, Err(String::from($RESULT)));
            }
        };
    }

    // basic positive path
    pos!(zero, "0", 0u8);
    pos!(one, "1", 1u8);
    pos!(neg_one, "-1", -1i8);
    pos!(limit, "255", 255u8);
    pos!(underscores, "1_000_000", 1_000_000u32);

    // basic positive path with Si suffix
    pos!(kilo, "1k", 1_000u16);
    pos!(kilo_caps, "1K", 1_000u16);
    pos!(mega, "1M", 1_000_000u32);
    pos!(giga, "1G", 1_000_000_000u64);
    pos!(tera, "1T", 1_000_000_000_000u64);
    pos!(peta, "1P", 1_000_000_000_000_000u64);
    pos!(exa, "1E", 1_000_000_000_000_000_000u64);
    pos!(zetta, "1Z", 1_000_000_000_000_000_000_000u128);
    pos!(yotta, "1Y", 1_000_000_000_000_000_000_000_000u128);

    pos!(trailing_1, "1k2", 1_200u16);
    pos!(trailing_2, "1k23", 1_230u16);
    pos!(trailing_3, "1k234", 1_234u16);
    neg!(trailing_4, "1k2345", u16, "not an integer");
    pos!(trailing_do_nothing, "1k000", 1_000u16);
    pos!(negative_trailing, "-1k234", -1_234i16);

    pos!(leading_2, "12k123", 12_123u16);
    pos!(leading_3, "123k123", 123_123u32);
    pos!(leading_4, "1234k123", 1_234_123i32);
    pos!(negative_leading, "-123k123", -123_123i32);

    pos!(dec_1, "1.2k", 1_200u16);
    pos!(dec_2, "1.23k", 1_230u16);
    pos!(dec_3, "1.234k", 1_234u16);
    neg!(dec_4, "1.2345k", u16, "not an integer");
    pos!(dec_do_nothing, "1.000k", 1_000u16);

    pos!(dec_ending_si, "1.k", 1_000u16);

    neg!(mixed_1, "1K23.45", u16, "not an integer");
    neg!(mixed_2, "1.23k45", u16, "invalid digit found in string");

    neg!(trailing_dec, "1.", u8, "invalid digit found in string");

    pos!(
        big,
        "1Y123456789987654321",
        1_123_456_789_987_654_321_000_000u128
    );

    neg!(leading_si, "k1", u16, "no value found before SI symbol");

    neg!(overflow, "1k", u8, "number too large to fit in target type");
    neg!(
        normal_overflow,
        "300",
        u8,
        "number too large to fit in target type"
    );

    neg!(multiple_suffix, "1kk", u16, "invalid digit found in string");
}

// integration tests with clap
#[cfg(test)]
mod integration {
    use super::*;

    #[derive(Parser)]
    struct Args {
        #[clap(long, value_parser=si_number::<u128>)]
        resistance: u128,
    }

    // positive path
    macro_rules! pos {
        ($NAME:ident, $VAL:expr, $RESULT:expr) => {
            #[test]
            fn $NAME() {
                let opt = Args::parse_from(&["", "--resistance", $VAL]);
                assert_eq!(opt.resistance, $RESULT);
            }
        };
    }

    // negative path
    macro_rules! neg {
        ($NAME:ident, $VAL:expr, $RESULT:expr) => {
            #[test]
            fn $NAME() {
                let opt = Args::try_parse_from(&["", "--resistance", $VAL]);
                match opt {
                    Err(e) => {
                        assert!(format!("{:?}", e).contains($RESULT));
                    }
                    _ => unreachable!(),
                };
            }
        };
    }

    pos!(simple_0, "1k123", 1123);
    pos!(simple_1, "456789k123", 456789123);
    pos!(simple_2, "1M1", 1_100_000);

    neg!(big, "999999999999999999999Y", "too large");
    neg!(invalid, "1k1k", "invalid digit");
    neg!(precise, "1k1111", "not an integer");
    neg!(leading_prefix, "k123", "no value found before SI symbol");
}
