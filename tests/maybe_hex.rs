use clap_num::maybe_hex;

#[cfg(test)]
mod basic {
    use super::*;

    // positive path
    macro_rules! pos {
        ($NAME:ident, $VAL:expr, $RESULT:expr) => {
            #[test]
            fn $NAME() {
                assert_eq!(maybe_hex($VAL), Ok($RESULT));
            }
        };
    }

    // negative path
    macro_rules! neg {
        ($NAME:ident, $VAL:expr, $RESULT:expr) => {
            #[test]
            fn $NAME() {
                let val: Result<u64, String> = maybe_hex($VAL);
                assert_eq!(val, Err(String::from($RESULT)));
            }
        };
    }

    pos!(simple, "123", 123u8);
    pos!(zero_dec, "0", 0u16);
    pos!(zero_hex, "0x0", 0u16);
    pos!(one_dec, "1", 1u64);
    pos!(one_hex, "0x1", 1u64);
    pos!(leading_zero, "001", 1u64);
    pos!(case, "0XABcDE", 703710u32);

    neg!(
        missing_suffix,
        "0x",
        "cannot parse integer from empty string"
    );
    neg!(dec_with_hex, "1A", "invalid digit found in string");
    neg!(non_hex_digit, "0x12G", "invalid digit found in string");
}
