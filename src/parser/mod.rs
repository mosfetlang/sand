pub use context::*;
pub use errors::*;
use jpar::Reader;
pub use traits::*;

pub mod commons;
mod context;
mod errors;
mod traits;

pub type ParserInput<'a> = Reader<'a, ParserError, ParserContext>;
pub type ParserResult<T> = jpar::ParserResult<T, ParserError>;
