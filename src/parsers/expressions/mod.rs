use jpar::branch::alternative;
use jpar::helpers::map_result;
use jpar::Span;

use crate::parsers::expressions::literals::Literal;
use crate::parsers::{ParserInput, ParserNode, ParserResult};

pub mod literals;

/// An expression.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expression<'a> {
    Literal(Literal<'a>),
}

impl<'a> Expression<'a> {
    // CONSTRUCTORS -----–-----–-----–-----–-----–-----–-----–-----–-----–-----

    // GETTERS -----–-----–-----–-----–-----–-----–-----–-----–-----–-----–----

    pub fn is_literal(&self) -> bool {
        matches!(self, Expression::Literal(_))
    }

    // METHODS -----–-----–-----–-----–-----–-----–-----–-----–-----–-----–----

    pub fn unwrap_literal(self) -> Literal<'a> {
        match self {
            Expression::Literal(v) => v,
        }
    }

    // STATIC METHODS -----–-----–-----–-----–-----–-----–-----–-----–-----–---

    /// Parses an expression.
    pub fn parse(input: &mut ParserInput<'a>) -> ParserResult<'a, Expression<'a>> {
        let mut parser = alternative((map_result(Literal::parse, |_, v| Expression::Literal(v)),));

        parser(input)
    }
}

impl<'a> ParserNode<'a> for Expression<'a> {
    fn span(&self) -> &Span<'a> {
        match self {
            Expression::Literal(v) => v.span(),
        }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use num_bigint::BigInt;
    use num_rational::BigRational;

    use crate::parsers::ParserContext;

    use super::*;

    #[test]
    fn test_parse_literal() {
        let context = ParserContext::default();
        let content = "215";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result = Expression::parse(&mut input).expect("The parser must succeed");
        assert_eq!(result.span_content(), content, "The content is incorrect");
        assert!(result.is_literal(), "The type of expression is incorrect");

        let result = result.unwrap_literal();
        assert!(result.is_number(), "The type of literal is incorrect");

        let result = result.unwrap_number();
        assert_eq!(
            result.value(),
            &BigRational::from(BigInt::from(215_u64)),
            "The value is incorrect"
        );
    }
}