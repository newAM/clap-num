use clap_num::maybe_bin;

#[cfg(test)]
mod basic {
    use super::*;

    // positive path
    macro_rules! pos {
        ($NAME:ident, $VAL:expr, $RESULT:expr) => {
            #[test]
            fn $NAME() {
                assert_eq!(maybe_bin($VAL), Ok($RESULT));
            }
        };
    }

    // negative path
    macro_rules! neg {
        ($NAME:ident, $VAL:expr, $RESULT:expr) => {
            #[test]
            fn $NAME() {
                let val: Result<u64, String> = maybe_bin($VAL);
                assert_eq!(val, Err(String::from($RESULT)));
            }
        };
    }

    pos!(simple, "123", 123u8);
    pos!(zero_dec, "0", 0u16);
    pos!(zero_bin, "0b0", 0u16);
    pos!(one_dec, "1", 1u64);
    pos!(one_bin, "0b1", 1u64);
    pos!(aa, "0b10101010", 0xaau64);
    pos!(leading_zero, "001", 1u64);

    neg!(
        missing_suffix,
        "0b",
        "cannot parse integer from empty string"
    );
    neg!(non_bin_digit, "0b12G", "invalid digit found in string");
}
