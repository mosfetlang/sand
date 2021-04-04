use doclog::Color;
use jpar::combinator::{end, optional};
use jpar::helpers::{and_then, ensure, error, map_result};
use jpar::sequence::{repeat, tuple};
use jpar::Span;

use crate::parsers::commons::Whitespace;
use crate::parsers::statements::Statement;
use crate::parsers::utils::{generate_error, generate_source_code};
use crate::parsers::{ParserError, ParserErrorKind, ParserInput, ParserNode, ParserResult};

/// A Sand module, normally a file.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Module<'a> {
    span: Span<'a>,
    statements: Vec<Statement<'a>>,
}

impl<'a> Module<'a> {
    // CONSTRUCTORS -----–-----–-----–-----–-----–-----–-----–-----–-----–-----

    /// Creates a new node without checking its values.
    ///
    /// # Safety
    ///
    /// Using this method can lead to an incorrect representation of a module.
    pub unsafe fn new_unchecked(span: Span<'a>, statements: Vec<Statement<'a>>) -> Module<'a> {
        Module { span, statements }
    }

    // GETTERS -----–-----–-----–-----–-----–-----–-----–-----–-----–-----–----

    pub fn statements(&self) -> &Vec<Statement<'a>> {
        &self.statements
    }

    pub fn statements_mut(&mut self) -> &mut Vec<Statement<'a>> {
        &mut self.statements
    }

    // SETTERS -----–-----–-----–-----–-----–-----–-----–-----–-----–-----–----

    /// Sets the span of the node without checking it.
    ///
    /// # Safety
    ///
    /// Using this method can lead to an incorrect representation of a module.
    pub unsafe fn set_span_unchecked(&mut self, span: Span<'a>) {
        self.span = span;
    }

    /// Sets the statements of the node without checking it.
    ///
    /// # Safety
    ///
    /// Using this method can lead to an incorrect representation of a module.
    pub unsafe fn set_statements(&mut self, statements: Vec<Statement<'a>>) {
        self.statements = statements;
    }

    // STATIC METHODS -----–-----–-----–-----–-----–-----–-----–-----–-----–---

    /// Parses a module.
    pub fn parse(input: &mut ParserInput<'a>) -> ParserResult<'a, Module<'a>> {
        let init_cursor = input.save_cursor();
        let mut is_first = true;
        let mut parser = map_result(
            tuple((
                repeat(
                    0..,
                    and_then(
                        tuple((optional(Whitespace::parse), Statement::parse)),
                        |input, (whitespace, statement)| {
                            if is_first {
                                is_first = false;
                                Ok(statement)
                            } else {
                                match whitespace {
                                    Some(whitespace) => {
                                        let ws_span = whitespace.span();
                                        if ws_span.start_cursor().line()
                                            == ws_span.end_cursor().line()
                                        {
                                            error(Self::error_two_statements_inline(
                                                input, statement,
                                            ))(input)
                                        } else {
                                            Ok(statement)
                                        }
                                    }
                                    None => error(Self::error_two_statements_inline(
                                        input, statement,
                                    ))(input),
                                }
                            }
                        },
                    ),
                ),
                optional(Whitespace::parse),
                ensure(end, |input| Self::error_unrecognized_eof(input)),
            )),
            |input, (statements, _, _)| Module {
                span: input.substring_to_current(&init_cursor),
                statements,
            },
        );

        parser(input)
    }

    pub fn error_two_statements_inline(
        input: &ParserInput<'a>,
        statement: Statement<'a>,
    ) -> ParserError<'a> {
        let span = statement.span();
        generate_error(
            ParserErrorKind::ModuleTwoStatementsInline,
            "Statements cannot be inline with others",
            |log| {
                generate_source_code(log, input, |doc| {
                    doc.highlight_cursor_message(
                        span.start_cursor().byte_offset(),
                        "Insert a line break here, e.g. '\\n'",
                        None,
                    )
                    .highlight_section(
                        span.start_cursor().byte_offset()..span.end_cursor().byte_offset(),
                        Some(Color::Magenta),
                    )
                })
            },
        )
    }

    pub fn error_unrecognized_eof(input: &ParserInput<'a>) -> ParserError<'a> {
        generate_error(
            ParserErrorKind::ModuleUnrecognizedEOF,
            "The module must finish here",
            |log| {
                generate_source_code(log, input, |doc| {
                    doc.highlight_cursor_message(
                        input.byte_offset(),
                        "The file is expected to end here",
                        None,
                    )
                    .highlight_section_message(
                        input.byte_offset()..input.content().len(),
                        "Unrecognized content (remove it)",
                        Some(Color::Magenta),
                    )
                })
            },
        )
    }
}

impl<'a> ParserNode<'a> for Module<'a> {
    fn span(&self) -> &Span<'a> {
        &self.span
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::parsers::{ParserContext, ParserInput};

    use super::*;

    #[test]
    fn test_parse_ok() {
        // Case 1: empty
        let context = ParserContext::default();
        let content = "";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result = Module::parse(&mut input).expect("[1] The parser must succeed");
        assert_eq!(
            result.span_content(),
            content,
            "[1] The content is incorrect"
        );
        assert_eq!(
            result.statements().len(),
            0,
            "[1] The number of statements is incorrect"
        );

        // Case 2: 1 statement without whites
        let context = ParserContext::default();
        let content = "const id = 3";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result = Module::parse(&mut input).expect("[2] The parser must succeed");
        assert_eq!(
            result.span_content(),
            content,
            "[2] The content is incorrect"
        );
        assert_eq!(
            result.statements().len(),
            1,
            "[2] The number of statements is incorrect"
        );

        // Case 3: 1 statement with whites
        let context = ParserContext::default();
        let content = "   \n\n  const id = 3   \n\n  ";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result = Module::parse(&mut input).expect("[3] The parser must succeed");
        assert_eq!(
            result.span_content(),
            content,
            "[3] The content is incorrect"
        );
        assert_eq!(
            result.statements().len(),
            1,
            "[3] The number of statements is incorrect"
        );

        // Case 4: 3 statement with whites
        let context = ParserContext::default();
        let content = "   \n\n  const id = 3   \nconst id = 3\n  const id = 3";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result = Module::parse(&mut input).expect("[4] The parser must succeed");
        assert_eq!(
            result.span_content(),
            content,
            "[4] The content is incorrect"
        );
        assert_eq!(
            result.statements().len(),
            3,
            "[4] The number of statements is incorrect"
        );
    }

    #[test]
    fn test_parse_error_not_found() {
        // The Module::parse always matches or returns another kind of error.
    }

    #[test]
    fn test_parse_error_two_statements_inline() {
        let context = ParserContext::default();
        let content = "const id = 3 const id = 3";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result = Module::parse(&mut input).expect_err("[1] The parser must not succeed");
        assert!(result.is_error(), "[1] The error is incorrect",);

        let (_cursor, error) = result.unwrap_error();
        assert!(
            matches!(error.kind, ParserErrorKind::ModuleTwoStatementsInline),
            "[1] The kind of error is incorrect",
        );

        // Print the error to test manually the generated template.
        println!("{}", error.log.to_ansi_text());
    }

    #[test]
    fn test_parse_error_unrecognized_eof() {
        let context = ParserContext::default();
        let content = "const identifier = 3 ++";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result = Module::parse(&mut input).expect_err("[1] The parser must not succeed");
        assert!(result.is_error(), "[1] The error is incorrect",);

        let (_cursor, error) = result.unwrap_error();
        assert!(
            matches!(error.kind, ParserErrorKind::ModuleUnrecognizedEOF),
            "[1] The kind of error is incorrect",
        );

        // Print the error to test manually the generated template.
        println!("{}", error.log.to_ansi_text());
    }
}
