use clap::Parser;
use clap_num::number_range;

// standalone basic tests
#[cfg(test)]
mod basic {
    use super::*;

    macro_rules! pos {
        ($NAME:ident, $VAL:expr, $MIN:expr, $MAX:expr, $RESULT:expr) => {
            #[test]
            fn $NAME() {
                assert_eq!(number_range($VAL, $MIN, $MAX), Ok($RESULT));
            }
        };
    }

    macro_rules! neg {
        ($NAME:ident, $VAL:expr, $MIN:expr, $MAX:expr, $RESULT:expr) => {
            #[test]
            fn $NAME() {
                assert_eq!(number_range($VAL, $MIN, $MAX), Err(String::from($RESULT)));
            }
        };
    }

    pos!(simple, "123", 12u8, 200u8, 123u8);
    pos!(zero, "0", 0u8, 0u8, 0u8);
    pos!(neg, "-1", -10i8, 10i8, -1);
    pos!(min_limit, "-5", -5i8, -5i8, -5i8);
    pos!(max_limit, "65535", 0, u16::MAX, u16::MAX);

    neg!(decimal, "1.1", -10i8, 10i8, "invalid digit found in string");
    neg!(min, "-1", 0i8, 0i8, "less than minimum of 0");
    neg!(max, "1", 0i8, 0i8, "exceeds maximum of 0");
    neg!(
        overflow,
        "256",
        0,
        std::u8::MAX,
        "number too large to fit in target type"
    );
    neg!(nan, "nan", 0, 0, "invalid digit found in string");

    #[test]
    #[should_panic]
    fn min_max_debug_assert() {
        let _ = number_range("", 2, 1);
    }
}

// integration tests with clap
#[cfg(test)]
mod integration {
    use super::*;

    fn human_livable_temperature(s: &str) -> Result<i8, String> {
        number_range(s, -40, 60)
    }

    #[derive(Parser, Debug)]
    struct Thermostat {
        #[clap(
            long,
            value_parser=human_livable_temperature,
            allow_hyphen_values=true
        )]
        temperature: i8,
    }

    // positive path
    macro_rules! pos {
        ($NAME:ident, $VAL:expr, $RESULT:expr) => {
            #[test]
            fn $NAME() {
                let opt = Thermostat::parse_from(&["", "--temperature", $VAL]);
                assert_eq!(opt.temperature, $RESULT);
            }
        };
    }

    // negative path
    macro_rules! neg {
        ($NAME:ident, $VAL:expr, $RESULT:expr) => {
            #[test]
            fn $NAME() {
                let opt = Thermostat::try_parse_from(&["", "--temperature", $VAL]);
                match opt {
                    Err(e) => {
                        assert!(format!("{:?}", e).contains($RESULT));
                    }
                    _ => unreachable!(),
                };
            }
        };
    }

    pos!(simple, "50", 50);
    pos!(zero, "0", 0);
    pos!(negative, "-30", -30);
    pos!(positive_limit, "60", 60);
    pos!(negative_limit, "-40", -40);

    neg!(too_small, "-41", "less than minimum of -40");
    neg!(too_large, "61", "exceeds maximum of 60");
}
