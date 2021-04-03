pub use config::*;
pub use context::*;
pub use errors::*;
use jpar::Reader;
pub use traits::*;
pub use warnings::*;

pub mod commons;
mod config;
mod constants;
mod context;
mod errors;
pub mod expressions;
mod traits;
mod utils;
mod warnings;

pub type ParserInput<'a> = Reader<'a, ParserError<'a>, ParserContext<'a>>;
pub type ParserResult<'a, T> = jpar::ParserResult<T, ParserError<'a>>;
