use std::fmt::{Display, Formatter};

use jpar::characters::read_text;
use jpar::helpers::map_result;
use jpar::sequence::{repeat_and_count, tuple_ignore};
use jpar::Span;

use crate::parsers::commons::Identifier;
use crate::parsers::{ParserInput, ParserNode, ParserResult};

pub static MODULE_PATH_SEPARATOR: &str = "::";

/// A single-line comment.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ModulePath<'a> {
    span: Span<'a>,
}

impl<'a> ModulePath<'a> {
    // CONSTRUCTORS -----–-----–-----–-----–-----–-----–-----–-----–-----–-----

    /// Creates a new node without checking its values.
    ///
    /// # Safety
    ///
    /// Using this method can lead to an incorrect representation of a module path.
    pub unsafe fn new_unchecked(span: Span<'a>) -> ModulePath<'a> {
        ModulePath { span }
    }

    // GETTERS -----–-----–-----–-----–-----–-----–-----–-----–-----–-----–----

    pub fn parts(&self) -> std::str::SplitTerminator<'a, &'static str> {
        self.span_content().split_terminator(MODULE_PATH_SEPARATOR)
    }

    // SETTERS -----–-----–-----–-----–-----–-----–-----–-----–-----–-----–----

    /// Sets the span of the node without checking it.
    ///
    /// # Safety
    ///
    /// Using this method can lead to an incorrect representation of a module path.
    pub unsafe fn set_span_unchecked(&mut self, span: Span<'a>) {
        self.span = span;
    }

    // STATIC METHODS -----–-----–-----–-----–-----–-----–-----–-----–-----–---

    /// Parses a module path.
    pub fn parse(input: &mut ParserInput<'a>) -> ParserResult<'a, ModulePath<'a>> {
        let init_cursor = input.save_cursor();
        let mut parser = map_result(
            repeat_and_count(
                1..,
                tuple_ignore((Identifier::parse, read_text(MODULE_PATH_SEPARATOR))),
            ),
            |input, _| ModulePath {
                span: input.substring_to_current(&init_cursor),
            },
        );

        parser(input)
    }
}

impl<'a> Display for ModulePath<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span.content())
    }
}

impl<'a> ParserNode<'a> for ModulePath<'a> {
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
        // Case 1: single
        let context = ParserContext::default();
        let content = "id1::";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result = ModulePath::parse(&mut input).expect("[1] The parser must succeed");
        assert_eq!(
            result.span_content(),
            content,
            "[1] The content is incorrect"
        );
        assert_eq!(
            result.parts().collect::<Vec<_>>(),
            vec!["id1"],
            "[1] The parts are incorrect"
        );

        // Case 2: single with terminator
        let context = ParserContext::default();
        let content = "id1::id_terminator";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result = ModulePath::parse(&mut input).expect("[2] The parser must succeed");
        assert_eq!(
            result.span_content(),
            "id1::",
            "[2] The content is incorrect"
        );
        assert_eq!(
            result.parts().collect::<Vec<_>>(),
            vec!["id1"],
            "[2] The parts are incorrect"
        );

        // Case 3: multiple
        let context = ParserContext::default();
        let content = "long::path::to::module";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result = ModulePath::parse(&mut input).expect("[3] The parser must succeed");
        assert_eq!(
            result.span_content(),
            "long::path::to::",
            "[3] The content is incorrect"
        );
        assert_eq!(
            result.parts().collect::<Vec<_>>(),
            vec!["long", "path", "to"],
            "[3] The parts are incorrect"
        );
    }

    #[test]
    fn test_parse_error_not_found() {
        // Case 1: other format
        let context = ParserContext::default();
        let content = "#[tag]";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result = ModulePath::parse(&mut input).expect_err("[1] The parser must not succeed");
        assert!(result.is_not_found(), "[1] The error is incorrect");

        // Case 2: empty
        let context = ParserContext::default();
        let content = "";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result = ModulePath::parse(&mut input).expect_err("[2] The parser must not succeed");
        assert!(result.is_not_found(), "[2] The error is incorrect");

        // Case 3: identifier
        let context = ParserContext::default();
        let content = "identifier";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result = ModulePath::parse(&mut input).expect_err("[3] The parser must not succeed");
        assert!(result.is_not_found(), "[3] The error is incorrect");

        // Case 4: single semicolon
        let context = ParserContext::default();
        let content = "id:a";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result = ModulePath::parse(&mut input).expect_err("[4] The parser must not succeed");
        assert!(result.is_not_found(), "[4] The error is incorrect");
    }
}
