use jpar::Span;

/// The trait all parser nodes implement.
pub trait ParserNode<'a> {
    fn span(&self) -> &Span<'a>;

    fn span_content(&self) -> &'a str {
        self.span().content()
    }
}
