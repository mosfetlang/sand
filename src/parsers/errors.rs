use doclog::Log;

/// The errors that parsers can throw.
#[derive(Debug, Clone)]
pub struct ParserError<'a> {
    pub kind: ParserErrorKind,
    pub log: Log<'a>,
}

/// The kind of error.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ParserErrorKind {
    NumberWithoutDigitsAfterDecimalSeparator,
    NumberWithoutDigitsAfterExponentToken,
    NumberTooBig,
    NumberTooBigExponent,

    ConstDeclarationWithoutIdentifier,
    ConstDeclarationWithoutAssignExpression,
    ConstDeclarationWithoutExpression,
}
