use criterion::measurement::WallTime;
use criterion::BenchmarkGroup;

use sand::parsers::commons::Comment;
use sand::parsers::{ParserContext, ParserInput, ParserNode};

static CONTENT: &str = "# Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin vel lacus sem. Integer vel neque bibendum, rhoncus lorem at, maximus felis. Curabitur in lacinia magna, sit amet semper eros. Mauris quis nulla at orci convallis vulputate. Proin tristique volutpat risus. Nam bibendum dictum semper. Ut pellentesque magna sed mauris vestibulum vulputate. Vestibulum nunc lorem, faucibus vitae massa in, accumsan ultrices sapien. Nam sit amet nulla sit amet elit luctus bibendum at in velit. Nulla facilisi. Pellentesque interdum est vel ipsum porttitor, et dignissim ipsum iaculis. Nulla dolor leo, pharetra at euismod eget, convallis et turpis.";

pub fn comment_bench(group: &mut BenchmarkGroup<WallTime>) {
    let context = ParserContext::default();
    let mut input = ParserInput::new_with_context_and_error(CONTENT, context);

    let comment = Comment::parse(&mut input).expect("The parser must succeed");
    assert_eq!(comment.span_content(), CONTENT, "The content is incorrect");
    assert_eq!(comment.message(), &CONTENT[2..], "The message is incorrect");

    group.bench_function("comment", |b| {
        b.iter(|| {
            let mut input =
                ParserInput::new_with_context_and_error(CONTENT, ParserContext::default());
            Comment::parse(&mut input).unwrap()
        });
    });
}
