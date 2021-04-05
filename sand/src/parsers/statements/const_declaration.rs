use std::cell::RefCell;
use std::fmt::{Display, Formatter};

use doclog::Color;
use jpar::characters::read_text;
use jpar::combinator::optional;
use jpar::helpers::{ensure, ignore_result, map_result, value_dyn};
use jpar::sequence::{tuple, tuple_ignore};
use jpar::{Cursor, Span};

use crate::parsers::commons::{Identifier, Whitespace};
use crate::parsers::expressions::Expression;
use crate::parsers::utils::{generate_error, generate_source_code};
use crate::parsers::{ParserError, ParserErrorKind, ParserInput, ParserNode, ParserResult};

pub static CONST_DECLARATION_KEYWORD: &str = "const";
pub static CONST_DECLARATION_ASSIGN_OPERATOR: &str = "=";

/// A constant declaration.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConstDeclaration<'a> {
    span: Span<'a>,
    identifier: Identifier<'a>,
    expression: Expression<'a>,
}

impl<'a> ConstDeclaration<'a> {
    // CONSTRUCTORS -----–-----–-----–-----–-----–-----–-----–-----–-----–-----

    /// Creates a new node without checking its values.
    ///
    /// # Safety
    ///
    /// Using this method can lead to an incorrect representation of constant declaration.
    pub unsafe fn new_unchecked(
        span: Span<'a>,
        identifier: Identifier<'a>,
        expression: Expression<'a>,
    ) -> ConstDeclaration<'a> {
        ConstDeclaration {
            span,
            identifier,
            expression,
        }
    }

    // GETTERS -----–-----–-----–-----–-----–-----–-----–-----–-----–-----–----

    pub fn identifier(&self) -> &Identifier<'a> {
        &self.identifier
    }

    pub fn identifier_mut(&mut self) -> &mut Identifier<'a> {
        &mut self.identifier
    }

    pub fn expression(&self) -> &Expression<'a> {
        &self.expression
    }

    pub fn expression_mut(&mut self) -> &mut Expression<'a> {
        &mut self.expression
    }

    // SETTERS -----–-----–-----–-----–-----–-----–-----–-----–-----–-----–----

    /// Sets the span of the node without checking it.
    ///
    /// # Safety
    ///
    /// Using this method can lead to an incorrect representation of a constant declaration.
    pub unsafe fn set_span_unchecked(&mut self, span: Span<'a>) {
        self.span = span;
    }

    /// Sets the identifier of the node without checking it.
    ///
    /// # Safety
    ///
    /// Using this method can lead to an incorrect representation of a constant declaration.
    pub unsafe fn set_identifier(&mut self, identifier: Identifier<'a>) {
        self.identifier = identifier;
    }

    /// Sets the expression of the node without checking it.
    ///
    /// # Safety
    ///
    /// Using this method can lead to an incorrect representation of a constant declaration.
    pub unsafe fn set_expression(&mut self, expression: Expression<'a>) {
        self.expression = expression;
    }

    // STATIC METHODS -----–-----–-----–-----–-----–-----–-----–-----–-----–---

    /// Parses a constant declaration.
    pub fn parse(input: &mut ParserInput<'a>) -> ParserResult<'a, ConstDeclaration<'a>> {
        let init_cursor = input.save_cursor();
        let post_identifier_cursor = RefCell::new(init_cursor.clone());
        let post_assign_operator_cursor = RefCell::new(init_cursor.clone());

        let mut parser = map_result(
            tuple((
                tuple_ignore((
                    Identifier::read_keyword(CONST_DECLARATION_KEYWORD),
                    ignore_result(optional(Whitespace::parse)),
                )),
                ensure(Identifier::parse, |input| {
                    Self::error_without_identifier(input, &init_cursor)
                }),
                value_dyn(|input| post_identifier_cursor.replace(input.save_cursor())),
                ensure(
                    map_result(
                        tuple((
                            ignore_result(optional(Whitespace::parse)),
                            read_text(CONST_DECLARATION_ASSIGN_OPERATOR),
                            ignore_result(optional(Whitespace::parse)),
                            value_dyn(|input| {
                                post_assign_operator_cursor.replace(input.save_cursor())
                            }),
                            ensure(Expression::parse, |input| {
                                Self::error_without_expression(
                                    input,
                                    &init_cursor,
                                    &*post_assign_operator_cursor.borrow(),
                                )
                            }),
                        )),
                        |_, v| v.4,
                    ),
                    |input| {
                        Self::error_without_assign_expression(
                            input,
                            &init_cursor,
                            &*post_identifier_cursor.borrow(),
                        )
                    },
                ),
            )),
            |input, (_, identifier, _, expression)| ConstDeclaration {
                span: input.substring_to_current(&init_cursor),
                identifier,
                expression,
            },
        );

        parser(input)
    }

    pub fn error_without_identifier(
        input: &ParserInput<'a>,
        init_cursor: &Cursor,
    ) -> ParserError<'a> {
        let end_index = init_cursor.byte_offset() + CONST_DECLARATION_KEYWORD.len();

        generate_error(
            ParserErrorKind::ConstDeclarationWithoutIdentifier,
            format!(
                "Missing identifier after the constant declaration keyword '{}'",
                CONST_DECLARATION_KEYWORD
            ),
            |log| {
                generate_source_code(log, input, |doc| {
                    doc.highlight_section(
                        init_cursor.byte_offset()..end_index,
                        Some(Color::Magenta),
                    )
                    .highlight_cursor_message(
                        input.byte_offset(),
                        "Add an identifier here",
                        None,
                    )
                })
            },
        )
    }

    pub fn error_without_assign_expression(
        input: &ParserInput<'a>,
        init_cursor: &Cursor,
        post_identifier_cursor: &Cursor,
    ) -> ParserError<'a> {
        generate_error(
            ParserErrorKind::ConstDeclarationWithoutAssignExpression,
            "Constant declarations require a value after their identifiers",
            |log| {
                generate_source_code(log, input, |doc| {
                    doc.highlight_section(
                        init_cursor.byte_offset()..post_identifier_cursor.byte_offset(),
                        Some(Color::Magenta),
                    )
                    .highlight_cursor_message(
                        input.byte_offset(),
                        "Add an expression here: = <expr>",
                        None,
                    )
                })
            },
        )
    }

    pub fn error_without_expression(
        input: &ParserInput<'a>,
        init_cursor: &Cursor,
        post_assign_operator_cursor: &Cursor,
    ) -> ParserError<'a> {
        generate_error(
            ParserErrorKind::ConstDeclarationWithoutExpression,
            format!(
                "Constant declarations require an expression after the assign operator '{}'",
                CONST_DECLARATION_ASSIGN_OPERATOR
            ),
            |log| {
                generate_source_code(log, input, |doc| {
                    doc.highlight_section(
                        init_cursor.byte_offset()..post_assign_operator_cursor.byte_offset(),
                        Some(Color::Magenta),
                    )
                    .highlight_cursor_message(
                        post_assign_operator_cursor.byte_offset(),
                        "Add an expression here",
                        None,
                    )
                })
            },
        )
    }
}

impl<'a> Display for ConstDeclaration<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            CONST_DECLARATION_KEYWORD,
            self.identifier,
            CONST_DECLARATION_ASSIGN_OPERATOR,
            self.expression
        )
    }
}

impl<'a> ParserNode<'a> for ConstDeclaration<'a> {
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
    use num_rational::BigRational;

    use crate::parsers::expressions::literals::Literal;
    use crate::parsers::ParserContext;

    use super::*;

    #[test]
    fn test_parse_ok() {
        // Case 1: normal
        let context = ParserContext::default();
        let content = "const identifier = 32; xxx";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result = ConstDeclaration::parse(&mut input).expect("[1] The parser must succeed");
        assert_eq!(
            result.span_content(),
            "const identifier = 32",
            "[1] The content is incorrect"
        );
        assert_eq!(
            result.identifier().span_content(),
            "identifier",
            "[1] The identifier is incorrect"
        );

        let expression = result.expression;
        assert!(
            matches!(expression, Expression::Literal(Literal::Number(_))),
            "[1] The expression is incorrect"
        );

        let number = expression.unwrap_literal().unwrap_number();
        assert_eq!(
            number.value(),
            &BigRational::from(BigInt::from(32_usize)),
            "[1] The number is incorrect"
        );
    }

    #[test]
    fn test_parse_error_not_found() {
        // Case 1: other element
        let context = ParserContext::default();
        let content = "# comment";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result =
            ConstDeclaration::parse(&mut input).expect_err("[1] The parser must not succeed");
        assert!(result.is_not_found(), "[1] The error is incorrect");

        // Case 2: empty
        let context = ParserContext::default();
        let content = "";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result =
            ConstDeclaration::parse(&mut input).expect_err("[2] The parser must not succeed");
        assert!(result.is_not_found(), "[2] The error is incorrect");

        // Case 3: not a keyword
        let context = ParserContext::default();
        let content = "constant identifier";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result =
            ConstDeclaration::parse(&mut input).expect_err("[3] The parser must not succeed");
        assert!(result.is_not_found(), "[3] The error is incorrect");
    }

    #[test]
    fn test_parse_error_without_identifier() {
        let context = ParserContext::default();
        let content = "const";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result =
            ConstDeclaration::parse(&mut input).expect_err("[1] The parser must not succeed");
        assert!(result.is_error(), "[1] The error is incorrect",);

        let (_cursor, error) = result.unwrap_error();
        assert!(
            matches!(
                error.kind,
                ParserErrorKind::ConstDeclarationWithoutIdentifier
            ),
            "[1] The kind of error is incorrect",
        );

        // Print the error to test manually the generated template.
        println!("{}", error.log.to_ansi_text());
    }

    #[test]
    fn test_parse_error_without_assign_expression() {
        let context = ParserContext::default();
        let content = "const identifier";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result =
            ConstDeclaration::parse(&mut input).expect_err("[1] The parser must not succeed");
        assert!(result.is_error(), "[1] The error is incorrect",);

        let (_cursor, error) = result.unwrap_error();
        assert!(
            matches!(
                error.kind,
                ParserErrorKind::ConstDeclarationWithoutAssignExpression
            ),
            "[1] The kind of error is incorrect",
        );

        // Print the error to test manually the generated template.
        println!("{}", error.log.to_ansi_text());
    }

    #[test]
    fn test_parse_error_without_expression() {
        let context = ParserContext::default();
        let content = "const identifier=";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result =
            ConstDeclaration::parse(&mut input).expect_err("[1] The parser must not succeed");
        assert!(result.is_error(), "[1] The error is incorrect",);

        let (_cursor, error) = result.unwrap_error();
        assert!(
            matches!(
                error.kind,
                ParserErrorKind::ConstDeclarationWithoutExpression
            ),
            "[1] The kind of error is incorrect",
        );

        // Print the error to test manually the generated template.
        println!("{}", error.log.to_ansi_text());
    }
}
