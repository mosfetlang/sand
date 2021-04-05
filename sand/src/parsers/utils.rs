use std::borrow::Cow;

use doclog::blocks::DocumentBlock;
use doclog::Log;

use crate::parsers::constants::{LOG_CODE_TITLE, LOG_ERROR_ID_TITLE, LOG_WARNING_ID_TITLE};
use crate::parsers::{ParserError, ParserErrorKind, ParserInput, ParserWarning, ParserWarningKind};

pub fn add_warning<'a, F>(
    input: &mut ParserInput<'a>,
    kind: ParserWarningKind,
    title: impl Into<Cow<'a, str>>,
    builder: F,
) where
    F: FnOnce(&mut ParserInput<'a>, Log<'a>) -> Log<'a>,
{
    let warning = ParserWarning {
        kind,
        log: builder(input, Log::warn().title(title, true, false)).indent(2, |log| {
            log.note(LOG_WARNING_ID_TITLE, format!("{:?}", kind))
        }),
    };
    input.context_mut().add_warning(warning);
}

pub fn generate_error<'a, F>(
    kind: ParserErrorKind,
    title: impl Into<Cow<'a, str>>,
    builder: F,
) -> ParserError<'a>
where
    F: FnOnce(Log<'a>) -> Log<'a>,
{
    ParserError {
        kind,
        log: builder(Log::error().title(title, true, false))
            .indent(2, |log| log.note(LOG_ERROR_ID_TITLE, format!("{:?}", kind))),
    }
}

pub fn generate_source_code<'a, F>(log: Log<'a>, input: &ParserInput<'a>, builder: F) -> Log<'a>
where
    F: FnOnce(DocumentBlock<'a>) -> DocumentBlock<'a>,
{
    log.indent(2, |log| {
        log.document(input.content(), |doc| {
            let doc = doc.title(LOG_CODE_TITLE);
            let doc = if let Some(file_path) = input.context().file_path() {
                doc.file_path(file_path.clone())
            } else {
                doc
            };

            builder(doc)
        })
    })
}
