#[macro_use]
extern crate criterion;
extern crate lang;

use criterion::black_box;
use criterion::Criterion;

use self::lang::lang::Lang;

fn variable_empty_decl(c: &mut Criterion) {
    c.bench_function("variable empty declaration", |b| {
        b.iter(|| Lang::new(black_box(Some("let variable: i32;"))))
    });
}

criterion_group!(benches, variable_empty_decl);
criterion_main!(benches);
