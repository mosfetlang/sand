use jpar::characters::{read_none_of0, read_text, ucd_single_line_whitespace0};
use jpar::combinator::verify;
use jpar::helpers::map_result;
use jpar::sequence::{preceded, tuple};
use jpar::verifiers::char_verifier;
use jpar::Span;

use crate::parsers::{ParserInput, ParserNode, ParserResult};

pub static COMMENT_START_TOKEN: &str = "#";

/// A single-line comment.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Comment<'a> {
    span: Span<'a>,
}

impl<'a> Comment<'a> {
    // CONSTRUCTORS -----–-----–-----–-----–-----–-----–-----–-----–-----–-----

    /// Creates a new node checking its input values.
    pub fn new(span: Span<'a>) -> Option<Comment<'a>> {
        // Check values.
        if !Self::check_span(&span) {
            return None;
        }

        Some(Comment { span })
    }

    /// Creates a new node without checking its values.
    ///
    /// # Safety
    ///
    /// Using this method can lead to an incorrect representation of a comment.
    pub unsafe fn new_unchecked(span: Span<'a>) -> Comment<'a> {
        Comment { span }
    }

    // GETTERS -----–-----–-----–-----–-----–-----–-----–-----–-----–-----–----

    pub fn message(&self) -> &'a str {
        &self.span_content()[COMMENT_START_TOKEN.len()..].trim()
    }

    // SETTERS -----–-----–-----–-----–-----–-----–-----–-----–-----–-----–----

    pub fn set_span(&mut self, span: Span<'a>) -> bool {
        if !Self::check_span(&span) {
            return false;
        }

        self.span = span;
        true
    }

    /// Sets the span of the node without checking it.
    ///
    /// # Safety
    ///
    /// Using this method can lead to an incorrect representation of a comment.
    pub unsafe fn set_span_unchecked(&mut self, span: Span<'a>) {
        self.span = span;
    }

    // STATIC METHODS -----–-----–-----–-----–-----–-----–-----–-----–-----–---

    /// Parses a single-line comment.
    pub fn parse(input: &mut ParserInput<'a>) -> ParserResult<Comment<'a>> {
        let init_cursor = input.save_cursor();
        let mut parser = map_result(
            verify(
                preceded(
                    read_text(COMMENT_START_TOKEN),
                    tuple((
                        ucd_single_line_whitespace0,
                        read_none_of0(char_verifier('\n')),
                    )),
                ),
                |_, (ws, content)| content.is_empty() || ws.len() == 1,
            ),
            |input, _| Comment {
                span: input.substring_to_current(&init_cursor),
            },
        );

        parser(input)
    }

    fn check_span(span: &Span<'a>) -> bool {
        let content = span.content();
        content.starts_with(COMMENT_START_TOKEN)
    }
}

impl<'a> ParserNode<'a> for Comment<'a> {
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
        // Case 1: terminated by eof
        let context = ParserContext::default();
        let content = "# This is a test  ";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let comment = Comment::parse(&mut input).expect("[1] The parser must succeed");
        assert_eq!(
            comment.span_content(),
            content,
            "[1] The content is incorrect"
        );
        assert_eq!(
            comment.message(),
            "This is a test",
            "[1] The message is incorrect"
        );

        // Case 2: terminated by \n
        let context = ParserContext::default();
        let content = "# This is a test\n content";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let comment = Comment::parse(&mut input).expect("[2] The parser must succeed");
        assert_eq!(
            comment.span_content(),
            "# This is a test",
            "[2] The content is incorrect"
        );
        assert_eq!(
            comment.message(),
            "This is a test",
            "[2] The message is incorrect"
        );

        // Case 3: empty
        let context = ParserContext::default();
        let content = "#";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let comment = Comment::parse(&mut input).expect("[3] The parser must succeed");
        assert_eq!(comment.span_content(), "#", "[3] The content is incorrect");
        assert_eq!(comment.message(), "", "[3] The message is incorrect");

        // Case 4: blank
        let context = ParserContext::default();
        let content = "#  \t  ";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let comment = Comment::parse(&mut input).expect("[4] The parser must succeed");
        assert_eq!(
            comment.span_content(),
            "#  \t  ",
            "[4] The content is incorrect"
        );
        assert_eq!(comment.message(), "", "[4] The message is incorrect");
    }

    #[test]
    fn test_parse_incorrect() {
        // Case 1: other format
        let context = ParserContext::default();
        let content = "#[tag]";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let comment = Comment::parse(&mut input).expect_err("[1] The parser must not succeed");
        assert!(comment.is_not_found(), "[1] The error is incorrect");

        // Case 2: empty
        let context = ParserContext::default();
        let content = "";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let comment = Comment::parse(&mut input).expect_err("[2] The parser must not succeed");
        assert!(comment.is_not_found(), "[2] The error is incorrect");
    }
}
