use doclog::Log;

/// The warnings that parsers can throw.
#[derive(Debug, Clone)]
pub struct ParserWarning<'a> {
    pub kind: ParserWarningKind,
    pub log: Log<'a>,
}

/// The kind of warning.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ParserWarningKind {
    NumberWithLeadingZeroes,
    NumberWithTrailingZeroes,
}
