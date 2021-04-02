use doclog::Log;

/// The error object that all parsers return.
#[derive(Debug, Clone)]
pub struct ParserError {
    log: Log,
}
