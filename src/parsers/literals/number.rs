use std::option::Option::Some;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Num, Zero};

use doclog::Color;
use jpar::characters::{decimal_digit1, read_any_of, read_char};
use jpar::combinator::optional;
use jpar::helpers::{and_then, consumed, ensure, map_result};
use jpar::sequence::tuple;
use jpar::verifiers::text_verifier;
use jpar::{Cursor, ParserResultError, Span};

use crate::parsers::utils::{add_warning, generate_error, generate_source_code};
use crate::parsers::{
    ParserError, ParserErrorKind, ParserInput, ParserNode, ParserResult, ParserWarningKind,
};

pub static NUMBER_DECIMAL_SEPARATOR: char = '.';
pub static NUMBER_DECIMAL_EXPONENT_TOKEN: &str = "eE";

/// A real number.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Number<'a> {
    span: Span<'a>,
    value: BigRational,
}

impl<'a> Number<'a> {
    // CONSTRUCTORS -----–-----–-----–-----–-----–-----–-----–-----–-----–-----

    /// Creates a new node without checking its values.
    ///
    /// # Safety
    ///
    /// Using this method can lead to an incorrect representation of a number.
    pub unsafe fn new_unchecked(span: Span<'a>, value: BigRational) -> Number<'a> {
        Number { span, value }
    }

    // GETTERS -----–-----–-----–-----–-----–-----–-----–-----–-----–-----–----

    pub fn value(&self) -> &BigRational {
        &self.value
    }

    // SETTERS -----–-----–-----–-----–-----–-----–-----–-----–-----–-----–----

    /// Sets the span of the node without checking it.
    ///
    /// # Safety
    ///
    /// Using this method can lead to an incorrect representation of a number.
    pub unsafe fn set_span_unchecked(&mut self, span: Span<'a>) {
        self.span = span;
    }

    // STATIC METHODS -----–-----–-----–-----–-----–-----–-----–-----–-----–---

    /// Parses a real number in decimal radix.
    pub fn parse_decimal(input: &mut ParserInput<'a>) -> ParserResult<'a, Number<'a>> {
        let init_cursor = input.save_cursor();

        let mut parser = and_then(
            tuple((
                decimal_digit1,
                map_result(
                    optional(tuple((
                        read_char(NUMBER_DECIMAL_SEPARATOR),
                        ensure(decimal_digit1, |input| {
                            Self::error_without_digits_after_decimal_separator(input, &init_cursor)
                        }),
                    ))),
                    |_, v| v.map(|(_, v)| v),
                ),
                map_result(
                    optional(tuple((
                        read_any_of(text_verifier(NUMBER_DECIMAL_EXPONENT_TOKEN)),
                        consumed(tuple((
                            optional(read_any_of(text_verifier("+-"))),
                            ensure(decimal_digit1, |input| {
                                Self::error_without_digits_after_exponent_token(input, &init_cursor)
                            }),
                        ))),
                    ))),
                    |_, v| v.map(|(_, v)| v),
                ),
            )),
            |input, (integer_part, decimal_part, exponent)| {
                Self::convert_to_number(input, &init_cursor, integer_part, decimal_part, exponent)
            },
        );

        parser(input)
    }

    pub fn convert_to_number(
        input: &mut ParserInput<'a>,
        init_cursor: &Cursor,
        integer_part: &'a str,
        decimal_part: Option<&'a str>,
        exponent: Option<&'a str>,
    ) -> ParserResult<'a, Number<'a>> {
        let integer_part_value = integer_part.trim_start_matches('0');
        let value = if integer_part_value.is_empty() {
            BigInt::zero()
        } else {
            BigInt::from_str_radix(integer_part_value, 10).unwrap()
        };

        let mut value = if let Some(decimal_part) = decimal_part {
            let decimal_part = decimal_part.trim_end_matches('0');
            if !decimal_part.is_empty() {
                let decimal_part_value = BigInt::from_str_radix(decimal_part, 10).unwrap();
                if decimal_part_value > BigInt::from(u32::MAX) {
                    return Err(ParserResultError::Error((
                        input.save_cursor(),
                        Self::error_too_big(input, init_cursor),
                    )));
                }

                let denom = BigInt::from(10_usize).pow(decimal_part.len() as u32);
                let numer = value * &denom + decimal_part_value;
                BigRational::new(numer, denom)
            } else {
                BigRational::from(value)
            }
        } else {
            BigRational::from(value)
        };

        if let Some(exponent) = exponent {
            let decimal_part_value = match i32::from_str_radix(exponent, 10) {
                Ok(v) => v,
                Err(_) => {
                    return Err(ParserResultError::Error((
                        input.save_cursor(),
                        Self::error_too_big_exponent(input, init_cursor, exponent),
                    )));
                }
            };
            value *= BigRational::from(BigInt::from(10_usize)).pow(decimal_part_value);
        }

        // Check warnings.
        Self::warning_leading_zeroes_integer_part(input, init_cursor, integer_part);
        Self::warning_leading_zeroes_exponent(input, init_cursor, exponent);
        Self::warning_trailing_zeroes(input, init_cursor, decimal_part);

        Ok(Number {
            span: input.substring_to_current(&init_cursor),
            value,
        })
    }

    pub fn error_too_big(input: &ParserInput<'a>, init_cursor: &Cursor) -> ParserError<'a> {
        generate_error(
            ParserErrorKind::NumberTooBig,
            "The number is too big to be handled",
            |log| {
                generate_source_code(log, input, |doc| {
                    doc.highlight_section(init_cursor.byte_offset()..input.byte_offset(), None)
                })
            },
        )
    }

    pub fn error_too_big_exponent(
        input: &ParserInput<'a>,
        init_cursor: &Cursor,
        exponent: &'a str,
    ) -> ParserError<'a> {
        let end_position = input.byte_offset() - exponent.len();
        generate_error(
            ParserErrorKind::NumberTooBigExponent,
            "The exponent of the number is too big to be handled",
            |log| {
                generate_source_code(log, input, |doc| {
                    doc.highlight_section(
                        init_cursor.byte_offset()..end_position,
                        Some(Color::Magenta),
                    )
                    .highlight_section(end_position..input.byte_offset(), None)
                })
                .indent(2, |log| {
                    log.note("Max value", format!("+{}", i32::MAX))
                        .note("Min value", format!("{}", i32::MIN))
                })
            },
        )
    }

    pub fn error_without_digits_after_decimal_separator(
        input: &ParserInput<'a>,
        init_cursor: &Cursor,
    ) -> ParserError<'a> {
        generate_error(
            ParserErrorKind::NumberWithoutDigitsAfterDecimalSeparator,
            format!(
                "At least one digit was expected after the decimal separator '{}'",
                NUMBER_DECIMAL_SEPARATOR
            ),
            |log| {
                generate_source_code(log, input, |doc| {
                    doc.highlight_section(
                        init_cursor.byte_offset()..input.byte_offset(),
                        Some(Color::Magenta),
                    )
                    .highlight_cursor_message(
                        input.byte_offset(),
                        "Add a digit here, e.g. 0",
                        None,
                    )
                })
            },
        )
    }

    pub fn error_without_digits_after_exponent_token(
        input: &ParserInput<'a>,
        init_cursor: &Cursor,
    ) -> ParserError<'a> {
        let content = input.substring_to_current(init_cursor).content();
        let exponent_token = if content.ends_with('+') || content.ends_with('-') {
            &content[content.len() - 2..]
        } else {
            &content[content.len() - 1..]
        };

        generate_error(
            ParserErrorKind::NumberWithoutDigitsAfterExponentToken,
            format!(
                "At least one digit was expected after the exponent token '{}'",
                exponent_token
            ),
            |log| {
                generate_source_code(log, input, |doc| {
                    doc.highlight_section(
                        init_cursor.byte_offset()..input.byte_offset(),
                        Some(Color::Magenta),
                    )
                    .highlight_cursor_message(
                        input.byte_offset(),
                        "Add a digit here, e.g. 0",
                        None,
                    )
                })
            },
        )
    }

    pub fn warning_leading_zeroes_integer_part(
        input: &mut ParserInput<'a>,
        init_cursor: &Cursor,
        integer_part: &'a str,
    ) {
        if input.context().ignore().number_leading_zeroes || integer_part == "0" {
            return;
        }

        let integer_part_trim = integer_part.trim_start_matches('0');

        if integer_part.len() != integer_part_trim.len() {
            let number_of_zeroes = integer_part.len()
                - integer_part_trim.len()
                - if integer_part_trim.is_empty() { 1 } else { 0 };

            let end_zeroes = init_cursor.byte_offset() + number_of_zeroes;

            add_warning(
                input,
                ParserWarningKind::NumberWithLeadingZeroes,
                "Leading zeroes in the integer part of a number are unnecessary",
                |input, log| {
                    generate_source_code(log, input, |doc| {
                        doc.highlight_section_message(
                            init_cursor.byte_offset()..end_zeroes,
                            if number_of_zeroes == 1 {
                                "Remove this zero"
                            } else {
                                "Remove these zeroes"
                            },
                            None,
                        )
                        .highlight_section(end_zeroes..input.byte_offset(), Some(Color::Magenta))
                    })
                },
            );
        }
    }

    pub fn warning_leading_zeroes_exponent(
        input: &mut ParserInput<'a>,
        init_cursor: &Cursor,
        exponent: Option<&'a str>,
    ) {
        if input.context().ignore().number_leading_zeroes {
            return;
        }

        let exponent = match exponent {
            Some(v) => v,
            None => return,
        };

        let exponent_trim_sign = exponent.trim_start_matches(['+', '-'].as_ref());
        let exponent_trim = exponent_trim_sign.trim_start_matches('0');

        if exponent_trim.is_empty() {
            // Case: 0..0 (not required decimal part)
            let exponent_index =
                input.byte_offset() - exponent.len() - 1 /* NUMBER_DECIMAL_EXPONENT_TOKEN */;

            add_warning(
                input,
                ParserWarningKind::NumberWithLeadingZeroes,
                "The exponent of the number is unnecessary",
                |input, log| {
                    generate_source_code(log, input, |doc| {
                        doc.highlight_section(
                            init_cursor.byte_offset()..exponent_index,
                            Some(Color::Magenta),
                        )
                        .highlight_section_message(
                            exponent_index..input.byte_offset(),
                            "Remove the exponent",
                            None,
                        )
                    })
                },
            );
        } else if exponent_trim_sign.len() != exponent_trim.len() {
            let exponent_index = input.byte_offset() - exponent_trim_sign.len();
            let number_of_zeroes = exponent_trim_sign.len() - exponent_trim.len();
            let end_zeroes = exponent_index + number_of_zeroes;

            add_warning(
                input,
                ParserWarningKind::NumberWithLeadingZeroes,
                "Leading zeroes in the exponent of a number are unnecessary",
                |input, log| {
                    generate_source_code(log, input, |doc| {
                        doc.highlight_section(
                            init_cursor.byte_offset()..exponent_index,
                            Some(Color::Magenta),
                        )
                        .highlight_section_message(
                            exponent_index..end_zeroes,
                            if number_of_zeroes == 1 {
                                "Remove this zero"
                            } else {
                                "Remove these zeroes"
                            },
                            None,
                        )
                        .highlight_section(end_zeroes..input.byte_offset(), Some(Color::Magenta))
                    })
                },
            );
        }
    }

    pub fn warning_trailing_zeroes(
        input: &mut ParserInput<'a>,
        init_cursor: &Cursor,
        decimal_part: Option<&'a str>,
    ) {
        if input.context().ignore().number_trailing_zeroes {
            return;
        }

        let decimal_part = match decimal_part {
            Some(v) => v,
            None => return,
        };

        let decimal_part_trim = decimal_part.trim_end_matches('0');
        let content = input.substring_to_current(init_cursor).content();

        if decimal_part_trim.is_empty() {
            // Case: 0..0 (not required decimal part)
            let decimal_index =
                init_cursor.byte_offset() + content.find(NUMBER_DECIMAL_SEPARATOR).unwrap();
            let exponent_index =
                NUMBER_DECIMAL_SEPARATOR.len_utf8() + decimal_index + decimal_part.len();

            add_warning(
                input,
                ParserWarningKind::NumberWithTrailingZeroes,
                "The decimal part of the number is unnecessary",
                |input, log| {
                    generate_source_code(log, input, |doc| {
                        let doc = doc
                            .highlight_section(
                                init_cursor.byte_offset()..decimal_index,
                                Some(Color::Magenta),
                            )
                            .highlight_section_message(
                                decimal_index..exponent_index,
                                "Remove the decimal part",
                                None,
                            );

                        if exponent_index < input.byte_offset() {
                            doc.highlight_section(
                                exponent_index..input.byte_offset(),
                                Some(Color::Magenta),
                            )
                        } else {
                            doc
                        }
                    })
                },
            );
        } else if decimal_part.len() != decimal_part_trim.len() {
            let decimal_index = init_cursor.byte_offset()
                + content.find(NUMBER_DECIMAL_SEPARATOR).unwrap()
                + NUMBER_DECIMAL_SEPARATOR.len_utf8();
            let number_of_zeroes = decimal_part.len() - decimal_part_trim.len();
            let start_zeroes = decimal_index + decimal_part_trim.len();
            let exponent_index = start_zeroes + number_of_zeroes;

            add_warning(
                input,
                ParserWarningKind::NumberWithTrailingZeroes,
                "Trailing zeroes in the decimal part of a number are unnecessary",
                |input, log| {
                    generate_source_code(log, input, |doc| {
                        let doc = doc
                            .highlight_section(
                                init_cursor.byte_offset()..start_zeroes,
                                Some(Color::Magenta),
                            )
                            .highlight_section_message(
                                start_zeroes..exponent_index,
                                if number_of_zeroes == 1 {
                                    "Remove this zero"
                                } else {
                                    "Remove these zeroes"
                                },
                                None,
                            );

                        if exponent_index < input.byte_offset() {
                            doc.highlight_section(
                                exponent_index..input.byte_offset(),
                                Some(Color::Magenta),
                            )
                        } else {
                            doc
                        }
                    })
                },
            );
        }
    }
}

impl<'a> ParserNode<'a> for Number<'a> {
    fn span(&self) -> &Span<'a> {
        &self.span
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use num_bigint::BigInt;

    use crate::parsers::ParserContext;

    use super::*;

    #[test]
    fn test_parse_decimal_integer_ok() {
        // Case 1: all digits
        let context = ParserContext::default();
        let content = "1234567890";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result = Number::parse_decimal(&mut input).expect("[1] The parser must succeed");
        assert_eq!(
            result.span_content(),
            content,
            "[1] The content is incorrect"
        );
        assert_eq!(
            result.value(),
            &BigRational::from(BigInt::from(1234567890_u64)),
            "[1] The value is incorrect"
        );
    }

    #[test]
    fn test_parse_decimal_real_ok() {
        // Case 1: integer
        let context = ParserContext::default();
        let content = "1234567890";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result = Number::parse_decimal(&mut input).expect("[1] The parser must succeed");
        assert_eq!(
            result.span_content(),
            content,
            "[1] The content is incorrect"
        );
        assert_eq!(
            result.value(),
            &BigRational::from(BigInt::from(1234567890_u64)),
            "[1] The value is incorrect"
        );

        // Case 2: with decimal part
        let context = ParserContext::default();
        let content = "1234567890.0123456789";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result = Number::parse_decimal(&mut input).expect("[2] The parser must succeed");
        assert_eq!(
            result.span_content(),
            content,
            "[2] The content is incorrect"
        );
        assert_eq!(
            result.value(),
            &BigRational::new(
                BigInt::from_str_radix("12345678900123456789", 10).unwrap(),
                BigInt::from_str_radix("10000000000", 10).unwrap(),
            ),
            "[2] The value is incorrect"
        );

        // Case 3: with exponent
        for exp_char in NUMBER_DECIMAL_EXPONENT_TOKEN.chars() {
            for sign_char in &["", "+", "-"] {
                let context = ParserContext::default();
                let content = format!("5{}{}10", exp_char, sign_char);
                let mut input = ParserInput::new_with_context_and_error(content.as_str(), context);

                let result = Number::parse_decimal(&mut input).expect(
                    format!("[3.{}.{}] The parser must succeed", exp_char, sign_char).as_str(),
                );
                assert_eq!(
                    result.span_content(),
                    content,
                    "[3.{}.{}] The content is incorrect",
                    exp_char,
                    sign_char
                );
                assert_eq!(
                    *result.value(),
                    BigRational::from(BigInt::from(5_usize))
                        * BigRational::from(BigInt::from(10_usize)).pow(if *sign_char == "-" {
                            -10
                        } else {
                            10
                        }),
                    "[3.{}.{}] The value is incorrect",
                    exp_char,
                    sign_char
                );
            }
        }

        // Case 4: all parts
        for exp_char in NUMBER_DECIMAL_EXPONENT_TOKEN.chars() {
            for sign_char in &["", "+", "-"] {
                let context = ParserContext::default();
                let content = format!("5.2564{}{}10", exp_char, sign_char);
                let mut input = ParserInput::new_with_context_and_error(content.as_str(), context);

                let result = Number::parse_decimal(&mut input).expect(
                    format!("[4.{}.{}] The parser must succeed", exp_char, sign_char).as_str(),
                );
                assert_eq!(
                    result.span_content(),
                    content,
                    "[4.{}.{}] The content is incorrect",
                    exp_char,
                    sign_char
                );
                assert_eq!(
                    *result.value(),
                    BigRational::new(
                        BigInt::from_str_radix("52564", 10).unwrap(),
                        BigInt::from_str_radix("10000", 10).unwrap(),
                    ) * BigRational::from(BigInt::from(10_usize)).pow(if *sign_char == "-" {
                        -10
                    } else {
                        10
                    }),
                    "[4.{}.{}] The value is incorrect",
                    exp_char,
                    sign_char
                );
            }
        }
    }

    #[test]
    fn test_parse_decimal_error_not_found() {
        // Case 1: other format
        let context = ParserContext::default();
        let content = "identifier";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result =
            Number::parse_decimal(&mut input).expect_err("[1] The parser must not succeed");
        assert!(result.is_not_found(), "[1] The error is incorrect");

        // Case 2: empty
        let context = ParserContext::default();
        let content = "";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result =
            Number::parse_decimal(&mut input).expect_err("[2] The parser must not succeed");
        assert!(result.is_not_found(), "[2] The error is incorrect");
    }

    #[test]
    fn test_parse_decimal_error_too_big() {
        // This error is generated when numbers are u32::MAX length, therefore the number must occupy
        // 4GiB of memory. Due to that, this error is tested manually.

        let context = ParserContext::default();
        let content = "100";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let init_cursor = input.save_cursor();
        input.read_text("100");

        let error = Number::error_too_big(&mut input, &init_cursor);
        assert!(
            matches!(error.kind, ParserErrorKind::NumberTooBig),
            "The kind of error is incorrect"
        );

        // Print the error to test manually the generated template.
        println!("{}", error.log.to_ansi_text());
    }

    #[test]
    fn test_parse_decimal_error_too_big_exponent() {
        // Case 1: positive overflow
        let context = ParserContext::default();
        let content = format!("1e{}", i32::MAX as i64 + 1);
        let mut input = ParserInput::new_with_context_and_error(content.as_str(), context);

        let result =
            Number::parse_decimal(&mut input).expect_err("[1] The parser must not succeed");
        assert!(result.is_error(), "[1] The error is incorrect");

        let (_cursor, error) = result.unwrap_error();
        assert!(
            matches!(error.kind, ParserErrorKind::NumberTooBigExponent),
            "[1] The kind of error is incorrect"
        );

        // Print the error to test manually the generated template.
        println!("{}", error.log.to_ansi_text());

        // Case 2: negative overflow
        let context = ParserContext::default();
        let content = format!("1e{}", i32::MIN as i64 - 1);
        let mut input = ParserInput::new_with_context_and_error(content.as_str(), context);

        let result =
            Number::parse_decimal(&mut input).expect_err("[2] The parser must not succeed");
        assert!(result.is_error(), "[2] The error is incorrect");

        let (_cursor, error) = result.unwrap_error();
        assert!(
            matches!(error.kind, ParserErrorKind::NumberTooBigExponent),
            "[2] The kind of error is incorrect"
        );

        // Print the error to test manually the generated template.
        println!("{}", error.log.to_ansi_text());
    }

    #[test]
    fn test_parse_decimal_error_without_digits_after_decimal_separator() {
        let context = ParserContext::default();
        let content = "1.";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result = Number::parse_decimal(&mut input).expect_err("The parser must not succeed");
        assert!(result.is_error(), "The error is incorrect");

        let (_cursor, error) = result.unwrap_error();
        assert!(
            matches!(
                error.kind,
                ParserErrorKind::NumberWithoutDigitsAfterDecimalSeparator
            ),
            "[1] The kind of error is incorrect"
        );

        // Print the error to test manually the generated template.
        println!("{}", error.log.to_ansi_text());
    }

    #[test]
    fn test_parse_decimal_error_without_digits_after_exponent_token() {
        for exp_char in NUMBER_DECIMAL_EXPONENT_TOKEN.chars() {
            for sign_char in &["", "+", "-"] {
                let context = ParserContext::default();
                let content = format!("1{}{}", exp_char, sign_char);
                let mut input = ParserInput::new_with_context_and_error(content.as_str(), context);

                let result = Number::parse_decimal(&mut input).expect_err(
                    format!("[1.{}.{}] The parser must not succeed", exp_char, sign_char).as_str(),
                );
                assert!(
                    result.is_error(),
                    "[1.{}.{}] The error is incorrect",
                    exp_char,
                    sign_char
                );

                let (_cursor, error) = result.unwrap_error();
                assert!(
                    matches!(
                        error.kind,
                        ParserErrorKind::NumberWithoutDigitsAfterExponentToken
                    ),
                    "[1.{}.{}] The kind of error is incorrect",
                    exp_char,
                    sign_char
                );

                // Print the error to test manually the generated template.
                println!("{}", error.log.to_ansi_text());
            }
        }
    }

    #[test]
    fn test_parse_decimal_warning_leading_zeroes_integer() {
        // Case: 0
        let context = ParserContext::default();
        let content = "0";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        Number::parse_decimal(&mut input).expect("[0] The parser must succeed");

        let warnings = input.context().warnings();
        assert!(
            warnings.is_empty(),
            "[0] The number of warnings is incorrect",
        );

        // Cases: leading zeroes
        for (i, content) in ["000", "01.123", "0001e+3"].iter().enumerate() {
            let i = i + 1;
            let context = ParserContext::default();
            let mut input = ParserInput::new_with_context_and_error(content, context);

            Number::parse_decimal(&mut input)
                .expect(format!("[{}] The parser must succeed", i).as_str());

            let warnings = input.context().warnings();
            assert_eq!(
                warnings.len(),
                1,
                "[{}] The number of warnings is incorrect",
                i
            );

            let warning = warnings.first().unwrap();

            assert!(
                matches!(warning.kind, ParserWarningKind::NumberWithLeadingZeroes),
                "[{}] The kind of warning is incorrect",
                i
            );

            // Print the warning to test manually the generated template.
            println!("{}", warning.log.to_ansi_text());
        }
    }

    #[test]
    fn test_parse_decimal_warning_leading_zeroes_exponent() {
        for (i, content) in [
            "1e01", "1e0001", "1e+01", "1e+0001", "1e-01", "1e-0001", "1e0", "1e000", "1e+0",
            "1e+000", "1e-0", "1e-000",
        ]
        .iter()
        .enumerate()
        {
            let context = ParserContext::default();
            let mut input = ParserInput::new_with_context_and_error(content, context);

            Number::parse_decimal(&mut input)
                .expect(format!("[{}] The parser must succeed", i).as_str());

            let warnings = input.context().warnings();
            assert_eq!(
                warnings.len(),
                1,
                "[{}] The number of warnings is incorrect",
                i
            );

            let warning = warnings.first().unwrap();

            assert!(
                matches!(warning.kind, ParserWarningKind::NumberWithLeadingZeroes),
                "[{}] The kind of warning is incorrect",
                i
            );

            // Print the warning to test manually the generated template.
            println!("{}", warning.log.to_ansi_text());
        }
    }

    #[test]
    fn test_parse_decimal_warning_trailing_zeroes() {
        for (i, content) in ["1.10", "1.1000", "1.0", "1.0000", "2.1000e4", "2.0000e4"]
            .iter()
            .enumerate()
        {
            let context = ParserContext::default();
            let mut input = ParserInput::new_with_context_and_error(content, context);

            Number::parse_decimal(&mut input)
                .expect(format!("[{}] The parser must succeed", i).as_str());

            let warnings = input.context().warnings();
            assert_eq!(
                warnings.len(),
                1,
                "[{}] The number of warnings is incorrect",
                i
            );

            let warning = warnings.first().unwrap();

            assert!(
                matches!(warning.kind, ParserWarningKind::NumberWithTrailingZeroes),
                "[{}] The kind of warning is incorrect",
                i
            );

            // Print the warning to test manually the generated template.
            println!("{}", warning.log.to_ansi_text());
        }
    }
}
