use std::ops::RangeInclusive;

use jpar::characters::{read_any_of, read_any_of0};
use jpar::helpers::map_result;
use jpar::sequence::tuple_ignore;
use jpar::verifiers::interval_verifier;
use jpar::{ParserResultError, Span};

use crate::parsers::{ParserInput, ParserNode, ParserResult};

// This classification is based on Swift's.
pub static HEAD_CHARS: &[RangeInclusive<char>] = &[
    'A'..='Z',
    '_'..='_',
    'a'..='z',
    '\u{00A8}'..='\u{00A8}',
    '\u{00AA}'..='\u{00AA}',
    '\u{00AD}'..='\u{00AD}',
    '\u{00AF}'..='\u{00AF}',
    '\u{00B2}'..='\u{00B5}',
    '\u{00B7}'..='\u{00BA}',
    '\u{00BC}'..='\u{00BE}',
    '\u{00C0}'..='\u{00D6}',
    '\u{00D8}'..='\u{00F6}',
    '\u{00F8}'..='\u{02FF}',
    '\u{0370}'..='\u{167F}',
    '\u{1681}'..='\u{180D}',
    '\u{180F}'..='\u{1DBF}',
    '\u{1E00}'..='\u{1FFF}',
    '\u{200B}'..='\u{200D}',
    '\u{202A}'..='\u{202E}',
    '\u{203F}'..='\u{2040}',
    '\u{2054}'..='\u{2054}',
    '\u{2060}'..='\u{20CF}',
    '\u{2100}'..='\u{218F}',
    '\u{2460}'..='\u{24FF}',
    '\u{2776}'..='\u{2793}',
    '\u{2C00}'..='\u{2DFF}',
    '\u{2E80}'..='\u{2FFF}',
    '\u{3004}'..='\u{3007}',
    '\u{3021}'..='\u{302F}',
    '\u{3031}'..='\u{D7FF}',
    '\u{F900}'..='\u{FD3D}',
    '\u{FD40}'..='\u{FDCF}',
    '\u{FDF0}'..='\u{FE1F}',
    '\u{FE30}'..='\u{FE44}',
    '\u{FE47}'..='\u{FFFD}',
    '\u{10000}'..='\u{1FFFD}',
    '\u{20000}'..='\u{2FFFD}',
    '\u{30000}'..='\u{3FFFD}',
    '\u{40000}'..='\u{4FFFD}',
    '\u{50000}'..='\u{5FFFD}',
    '\u{60000}'..='\u{6FFFD}',
    '\u{70000}'..='\u{7FFFD}',
    '\u{80000}'..='\u{8FFFD}',
    '\u{90000}'..='\u{9FFFD}',
    '\u{A0000}'..='\u{AFFFD}',
    '\u{B0000}'..='\u{BFFFD}',
    '\u{C0000}'..='\u{CFFFD}',
    '\u{D0000}'..='\u{DFFFD}',
    '\u{E0000}'..='\u{EFFFD}',
];

// This classification is based on Swift's.
pub static BODY_CHARS: &[RangeInclusive<char>] = &[
    '0'..='9',
    '\u{0300}'..='\u{036F}',
    '\u{1DC0}'..='\u{1DFF}',
    '\u{20D0}'..='\u{20FF}',
    '\u{FE20}'..='\u{FE2F}',
];

/// A valid identifier.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Identifier<'a> {
    span: Span<'a>,
}

impl<'a> Identifier<'a> {
    // CONSTRUCTORS -----–-----–-----–-----–-----–-----–-----–-----–-----–-----

    /// Creates a new node without checking its values.
    ///
    /// # Safety
    ///
    /// Using this method can lead to an incorrect representation of an identifier.
    pub unsafe fn new_unchecked(span: Span<'a>) -> Identifier<'a> {
        Identifier { span }
    }

    // SETTERS -----–-----–-----–-----–-----–-----–-----–-----–-----–-----–----

    /// Sets the span of the node without checking it.
    ///
    /// # Safety
    ///
    /// Using this method can lead to an incorrect representation of an identifier.
    pub unsafe fn set_span_unchecked(&mut self, span: Span<'a>) {
        self.span = span;
    }

    // STATIC METHODS -----–-----–-----–-----–-----–-----–-----–-----–-----–---

    /// Parses an identifier.
    pub fn parse(input: &mut ParserInput<'a>) -> ParserResult<'a, Identifier<'a>> {
        let verifier_head = interval_verifier(HEAD_CHARS);
        let verifier_body = interval_verifier(BODY_CHARS);
        let verifier_both = |i, v| verifier_head(i, v) || verifier_body(i, v);

        let init_cursor = input.save_cursor();
        let mut parser = map_result(
            tuple_ignore((
                read_any_of(interval_verifier(HEAD_CHARS)),
                read_any_of0(verifier_both),
            )),
            |input, _| Identifier {
                span: input.substring_to_current(&init_cursor),
            },
        );

        parser(input)
    }

    /// Reads a keyword ensuring it does not belong to other words.
    ///
    /// For example: this parser matches 'key' in 'key' but not in 'keyword'.
    #[allow(clippy::result_unit_err)]
    pub fn read_keyword(
        keyword: &'a str,
    ) -> impl FnMut(&mut ParserInput<'a>) -> ParserResult<'a, ()> {
        move |input| {
            let init_cursor = input.save_cursor();
            let id = Self::parse(input)?;

            if id.span_content() == keyword {
                Ok(())
            } else {
                input.restore(init_cursor);
                Err(ParserResultError::NotFound)
            }
        }
    }
}

impl<'a> ParserNode<'a> for Identifier<'a> {
    fn span(&self) -> &Span<'a> {
        &self.span
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::parsers::ParserContext;

    use super::*;

    #[test]
    fn test_parse_ok() {
        // Case 1: head
        let context = ParserContext::default();
        let content = "a";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result = Identifier::parse(&mut input).expect("[1] The parser must succeed");
        assert_eq!(
            result.span_content(),
            content,
            "[1] The content is incorrect"
        );

        // Case 2: head head+
        let context = ParserContext::default();
        let content = "thisIsATest";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result = Identifier::parse(&mut input).expect("[2] The parser must succeed");
        assert_eq!(
            result.span_content(),
            content,
            "[2] The content is incorrect"
        );

        // Case 3: head body+
        let context = ParserContext::default();
        let content = "test0123845";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result = Identifier::parse(&mut input).expect("[3] The parser must succeed");
        assert_eq!(
            result.span_content(),
            content,
            "[3] The content is incorrect"
        );

        // Case 4: special characters
        let context = ParserContext::default();
        let content = "\u{FDF0}\u{20D0}";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result = Identifier::parse(&mut input).expect("[4] The parser must succeed");
        assert_eq!(
            result.span_content(),
            content,
            "[4] The content is incorrect"
        );
    }

    #[test]
    fn test_parse_error_not_found() {
        // Case 1: other element
        let context = ParserContext::default();
        let content = "# comment";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result = Identifier::parse(&mut input).expect_err("[1] The parser must not succeed");
        assert!(result.is_not_found(), "[1] The error is incorrect");

        // Case 2: empty
        let context = ParserContext::default();
        let content = "";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result = Identifier::parse(&mut input).expect_err("[2] The parser must not succeed");
        assert!(result.is_not_found(), "[2] The error is incorrect");
    }

    #[test]
    fn test_read_keyword() {
        // Case 1: ok
        let context = ParserContext::default();
        let content = "const x";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let mut parser = Identifier::read_keyword("const");
        let _ = parser(&mut input).expect("[1] The parser must succeed");
        assert_eq!(
            input.byte_offset(),
            "const".len(),
            "[1] The byte_offset is incorrect"
        );

        // Case 2: nok
        let context = ParserContext::default();
        let content = "constant";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let mut parser = Identifier::read_keyword("const");
        let result = parser(&mut input).expect_err("[2] The parser must not succeed");
        assert!(result.is_not_found(), "[2] The error is incorrect");
    }
}
