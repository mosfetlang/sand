use jpar::branch::alternative;
use jpar::helpers::map_result;
use jpar::Span;
pub use number::*;

use crate::parsers::{ParserInput, ParserNode, ParserResult};

mod number;

/// A literal value.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Literal<'a> {
    Number(Number<'a>),
}

impl<'a> Literal<'a> {
    // CONSTRUCTORS -----–-----–-----–-----–-----–-----–-----–-----–-----–-----

    // GETTERS -----–-----–-----–-----–-----–-----–-----–-----–-----–-----–----

    pub fn is_number(&self) -> bool {
        matches!(self, Literal::Number(_))
    }

    // METHODS -----–-----–-----–-----–-----–-----–-----–-----–-----–-----–----

    pub fn unwrap_number(self) -> Number<'a> {
        match self {
            Literal::Number(v) => v,
        }
    }

    // STATIC METHODS -----–-----–-----–-----–-----–-----–-----–-----–-----–---

    /// Parses literal value.
    pub fn parse(input: &mut ParserInput<'a>) -> ParserResult<'a, Literal<'a>> {
        let mut parser =
            alternative((map_result(Number::parse_decimal, |_, v| Literal::Number(v)),));

        parser(input)
    }
}

impl<'a> ParserNode<'a> for Literal<'a> {
    fn span(&self) -> &Span<'a> {
        match self {
            Literal::Number(v) => v.span(),
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
    fn test_parse_number() {
        let context = ParserContext::default();
        let content = "215";
        let mut input = ParserInput::new_with_context_and_error(content, context);

        let result = Literal::parse(&mut input).expect("The parser must succeed");
        assert_eq!(result.span_content(), content, "The content is incorrect");
        assert!(result.is_number(), "The type of literal is incorrect");

        let result = result.unwrap_number();
        assert_eq!(
            result.value(),
            &BigRational::from(BigInt::from(215_u64)),
            "The value is incorrect"
        );
    }
}
