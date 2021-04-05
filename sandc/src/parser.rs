use sand::parsers::{Module, ParserContext, ParserIgnoreConfig, ParserInput, ParserResult};

/// Parses a Sand file.
pub fn parse_file<'a>(content: &'a str, file_path: &'a str) -> ParserResult<'a, Module<'a>> {
    let parser_context = ParserContext::new(Some(file_path), ParserIgnoreConfig::new());
    let mut reader = ParserInput::new_with_context_and_error(content, parser_context);

    Module::parse(&mut reader)
}
