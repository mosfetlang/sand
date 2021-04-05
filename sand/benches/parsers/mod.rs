#[macro_use]
extern crate criterion;

mod commons;

criterion_group!(benches, commons::commons_benches);
criterion_main!(benches);
