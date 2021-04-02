use criterion::Criterion;

mod comments;

pub fn commons_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parsers/Commons");

    comments::comment_bench(&mut group);

    group.finish();
}
