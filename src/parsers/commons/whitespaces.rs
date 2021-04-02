use jpar::branch::alternative;
use jpar::characters::ucd_whitespace1;
use jpar::helpers::{ignore_result, map_result};
use jpar::sequence::repeat_and_count;
use jpar::Span;

use crate::parsers::commons::Comment;
use crate::parsers::{ParserInput, ParserNode, ParserResult};

/// A multiline whitespace that can include comments.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Whitespace<'a> {
    span: Span<'a>,
}

impl<'a> Whitespace<'a> {
    // CONSTRUCTORS -----–-----–-----–-----–-----–-----–-----–-----–-----–-----

    /// Creates a new node without checking its values.
    ///
    /// # Safety
    ///
    /// Using this method can lead to an incorrect representation of a whitespace section.
    pub unsafe fn new_unchecked(span: Span<'a>) -> Whitespace<'a> {
        Whitespace { span }
    }

    // SETTERS -----–-----–-----–-----–-----–-----–-----–-----–-----–-----–----

    /// Sets the span of the node without checking it.
    ///
    /// # Safety
    ///
    /// Using this method can lead to an incorrect representation of a whitespace section.
    pub unsafe fn set_span_unchecked(&mut self, span: Span<'a>) {
        self.span = span;
    }

    // STATIC METHODS -----–-----–-----–-----–-----–-----–-----–-----–-----–---

    /// Parses a multiline whitespace that can include comments.
    pub fn parse(input: &mut ParserInput<'a>) -> ParserResult<Whitespace<'a>> {
        let init_cursor = input.save_cursor();
        let mut parser = map_result(
            repeat_and_count(
                1..,
                alternative((
                    ignore_result(ucd_whitespace1),
                    ignore_result(Comment::parse),
                )),
            ),
            |input, _| Whitespace {
                span: input.substring_to_current(&init_cursor),
            },
        );

        parser(input)
    }
}

impl<'a> ParserNode<'a> for Whitespace<'a> {
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
        // Case 1: only whitespaces
        let context = ParserContext::default();
        let content = "  \t\n  \r\nidentifier";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let comment = Whitespace::parse(&mut input).expect("[1] The parser must succeed");
        assert_eq!(
            comment.span_content(),
            "  \t\n  \r\n",
            "[1] The content is incorrect"
        );

        // Case 2: whitespaces comment whitespaces
        let context = ParserContext::default();
        let content = "  # This is a test\n content";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let comment = Whitespace::parse(&mut input).expect("[2] The parser must succeed");
        assert_eq!(
            comment.span_content(),
            "  # This is a test\n ",
            "[2] The content is incorrect"
        );

        // Case 3: comment whitespaces
        let context = ParserContext::default();
        let content = "# This is a test\n\n\r\n content";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let comment = Whitespace::parse(&mut input).expect("[3] The parser must succeed");
        assert_eq!(
            comment.span_content(),
            "# This is a test\n\n\r\n ",
            "[3] The content is incorrect"
        );
    }

    #[test]
    fn test_parse_incorrect() {
        // Case 1: other element
        let context = ParserContext::default();
        let content = "identifier";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let comment = Whitespace::parse(&mut input).expect_err("[1] The parser must not succeed");
        assert!(comment.is_not_found(), "[1] The error is incorrect");

        // Case 2: empty
        let context = ParserContext::default();
        let content = "";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let comment = Whitespace::parse(&mut input).expect_err("[2] The parser must not succeed");
        assert!(comment.is_not_found(), "[2] The error is incorrect");
    }
}
