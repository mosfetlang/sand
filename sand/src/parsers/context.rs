use std::borrow::Cow;

use crate::parsers::{ParserIgnoreConfig, ParserWarning};

/// The context object that carries all information of the parser.
#[derive(Debug)]
pub struct ParserContext<'a> {
    file_path: Option<Cow<'a, str>>,
    warnings: Vec<ParserWarning<'a>>,
    ignore: ParserIgnoreConfig,
}

impl<'a> ParserContext<'a> {
    // CONSTRUCTORS -----------------------------------------------------------

    /// Builds a new `ParserContext` with the default configuration.
    pub fn new(
        file_path: Option<impl Into<Cow<'a, str>>>,
        ignore: ParserIgnoreConfig,
    ) -> ParserContext<'a> {
        ParserContext {
            file_path: file_path.map(|v| v.into()),
            warnings: Vec::new(),
            ignore,
        }
    }

    // GETTERS ----------------------------------------------------------------

    pub fn file_path(&self) -> &Option<Cow<'a, str>> {
        &self.file_path
    }

    pub fn warnings(&self) -> &Vec<ParserWarning<'a>> {
        &self.warnings
    }

    pub fn ignore(&self) -> &ParserIgnoreConfig {
        &self.ignore
    }

    // METHODS ----------------------------------------------------------------

    pub fn add_warning(&mut self, warning: ParserWarning<'a>) {
        self.warnings.push(warning);
    }
}

impl<'a> Default for ParserContext<'a> {
    fn default() -> Self {
        Self::new(None::<&'static str>, ParserIgnoreConfig::default())
    }
}
