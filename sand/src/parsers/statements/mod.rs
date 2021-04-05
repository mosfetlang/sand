use std::fmt::{Display, Formatter};

pub use const_declaration::*;
use jpar::branch::alternative;
use jpar::helpers::map_result;
use jpar::Span;

use crate::parsers::{ParserInput, ParserNode, ParserResult};

mod const_declaration;

/// A program statement.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Statement<'a> {
    ConstDeclaration(ConstDeclaration<'a>),
}

impl<'a> Statement<'a> {
    // CONSTRUCTORS -----–-----–-----–-----–-----–-----–-----–-----–-----–-----

    // GETTERS -----–-----–-----–-----–-----–-----–-----–-----–-----–-----–----

    pub fn is_const_declaration(&self) -> bool {
        matches!(self, Statement::ConstDeclaration(_))
    }

    // METHODS -----–-----–-----–-----–-----–-----–-----–-----–-----–-----–----

    pub fn unwrap_const_declaration(self) -> ConstDeclaration<'a> {
        match self {
            Statement::ConstDeclaration(v) => v,
        }
    }

    // STATIC METHODS -----–-----–-----–-----–-----–-----–-----–-----–-----–---

    /// Parses an expression.
    pub fn parse(input: &mut ParserInput<'a>) -> ParserResult<'a, Statement<'a>> {
        let mut parser = alternative((map_result(ConstDeclaration::parse, |_, v| {
            Statement::ConstDeclaration(v)
        }),));

        parser(input)
    }
}

impl<'a> Display for Statement<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Statement::ConstDeclaration(v) => v.fmt(f),
        }
    }
}

impl<'a> ParserNode<'a> for Statement<'a> {
    fn span(&self) -> &Span<'a> {
        match self {
            Statement::ConstDeclaration(v) => v.span(),
        }
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
    fn test_parse_const_declaration() {
        let context = ParserContext::default();
        let content = "const identifier = 23";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result = Statement::parse(&mut input).expect("The parser must succeed");
        assert_eq!(result.span_content(), content, "The content is incorrect");
        assert!(
            result.is_const_declaration(),
            "The type of expression is incorrect"
        );
    }
}
