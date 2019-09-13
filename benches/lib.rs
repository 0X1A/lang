// This file is auto-generated. Please do not edit it manually.

#[macro_use]
extern crate criterion;
extern crate lang;

use self::lang::lang::Lang;
use criterion::{black_box, Criterion};

fn array_i64_variable_declaration(c: &mut Criterion) {
    c.bench_function("Array<i64> Variable declaration", |b| {
        b.iter(|| Lang::new(black_box(Some("let i: Array<i64>;"))))
    });
}
fn array_i64_variable_declaration_and_assignment(c: &mut Criterion) {
    c.bench_function("Array<i64> Variable declaration and assignment", |b| {
        b.iter(|| Lang::new(black_box(Some("let i: Array<i64> = [0, 1, 2];"))))
    });
}
fn array_i64_variable_declaration_empty(c: &mut Criterion) {
    c.bench_function("Array<i64> Variable declaration empty", |b| {
        b.iter(|| Lang::new(black_box(Some("let i: Array<i64> = [];"))))
    });
}
fn array_i64_variable_re_assignment(c: &mut Criterion) {
    c.bench_function("Array<i64> Variable re-assignment", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "let i: Array<i64> = [];
            i = [0, 1, 2];",
            )))
        })
    });
}
fn array_i64_variable_re_assignment_failure(c: &mut Criterion) {
    c.bench_function("Array<i64> Variable re-assignment failure", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "let i: Array<i64> = [];
            i = [0.00, 1.00, 2.00];",
            )))
        })
    });
}
fn for_loop(c: &mut Criterion) {
    c.bench_function("For loop", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        for (let i: i32 = 0; i < 10; i = i + 1) {
            print i;
        }
        for (let b: bool = false; b == true; b = false) {
            print b;
        }
        ",
            )))
        })
    });
}
fn struct_declaration(c: &mut Criterion) {
    c.bench_function("Struct declaration", |b| {
        b.iter(|| Lang::new(black_box(Some("struct TestStruct {}"))))
    });
}
fn struct_declaration_failure(c: &mut Criterion) {
    c.bench_function("Struct declaration failure", |b| {
        b.iter(|| Lang::new(black_box(Some("struct TestStruct {{}"))))
    });
}
fn struct_declaration_with_fields(c: &mut Criterion) {
    c.bench_function("Struct declaration with fields", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        struct TestStruct {
            field0: i64,
            field1: f64,
            field2: bool
        }
        ",
            )))
        })
    });
}
fn struct_with_field_access_failure(c: &mut Criterion) {
    c.bench_function("Struct with field access failure", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        struct TestStruct {}

        let instance: TestStruct = TestStruct();
        instance.field_doesnt_exist;
        ",
            )))
        })
    });
}
fn struct_with_fields_instatiation(c: &mut Criterion) {
    c.bench_function("Struct with fields instatiation", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        struct TestStruct {
            field0: i64,
            field1: f64,
            field2: bool
        }

        let instance: TestStruct = TestStruct();
        ",
            )))
        })
    });
}
fn struct_with_fields_instatiation_and_field_assignment(c: &mut Criterion) {
    c.bench_function(
        "Struct with fields instatiation and field assignment",
        |b| {
            b.iter(|| {
                Lang::new(black_box(Some(
                    "
        struct TestStruct {
            field0: i64,
            field1: f64,
            field2: bool
        }

        let instance: TestStruct = TestStruct();
        instance.field0 = 0;
        instance.field1 = 1.00;
        instance.field2 = false;
        ",
                )))
            })
        },
    );
}
fn struct_with_impl(c: &mut Criterion) {
    c.bench_function("Struct with impl", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        struct TestStruct {
        }

        impl TestStruct {
            fn hello() -> () {
                print \"Hello world\";
            }
        }

        let instance: TestStruct = TestStruct();
        instance.hello();
        ",
            )))
        })
    });
}
fn struct_with_method_call_failure(c: &mut Criterion) {
    c.bench_function("Struct with method call failure", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        struct TestStruct {
        }

        impl TestStruct {
        }

        let instance: TestStruct = TestStruct();
        instance.hello();
        ",
            )))
        })
    });
}
fn while_loop(c: &mut Criterion) {
    c.bench_function("While Loop", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        let b: bool = true;
        let i: i32 = 0;
        while (b) {
            i = i + 1;
            if (i == 10) {
                b = false;
            }
        }
        ",
            )))
        })
    });
}
fn f64_variable_declaration(c: &mut Criterion) {
    c.bench_function("f64 Variable declaration", |b| {
        b.iter(|| Lang::new(black_box(Some("let i: f64;"))))
    });
}
fn f64_variable_declaration_and_assignment(c: &mut Criterion) {
    c.bench_function("f64 Variable declaration and assignment", |b| {
        b.iter(|| Lang::new(black_box(Some("let i: f64 = 0.00;"))))
    });
}
fn f64_variable_re_assignment(c: &mut Criterion) {
    c.bench_function("f64 Variable re-assignment", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "let i: f64 = 0.00;
            i = 100.00;",
            )))
        })
    });
}
fn f64_variable_re_assignment_failure(c: &mut Criterion) {
    c.bench_function("f64 Variable re-assignment failure", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "let i: f64 = 0;
        i = 100;",
            )))
        })
    });
}
fn i64_variable_declaration(c: &mut Criterion) {
    c.bench_function("i64 Variable declaration", |b| {
        b.iter(|| Lang::new(black_box(Some("let i: i64;"))))
    });
}
fn i64_variable_declaration_and_assignment(c: &mut Criterion) {
    c.bench_function("i64 Variable declaration and assignment", |b| {
        b.iter(|| Lang::new(black_box(Some("let i: i64 = 0;"))))
    });
}
fn i64_variable_re_assignment(c: &mut Criterion) {
    c.bench_function("i64 Variable re-assignment", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "let i: i64 = 0;
            i = 100;",
            )))
        })
    });
}
fn i64_variable_re_assignment_failure(c: &mut Criterion) {
    c.bench_function("i64 Variable re-assignment failure", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "let i: i64 = 0;
        i = 100.00;",
            )))
        })
    });
}
criterion_group!(
    lang_benches,
    array_i64_variable_declaration,
    array_i64_variable_declaration_and_assignment,
    array_i64_variable_declaration_empty,
    array_i64_variable_re_assignment,
    array_i64_variable_re_assignment_failure,
    for_loop,
    struct_declaration,
    struct_declaration_failure,
    struct_declaration_with_fields,
    struct_with_field_access_failure,
    struct_with_fields_instatiation,
    struct_with_fields_instatiation_and_field_assignment,
    struct_with_impl,
    struct_with_method_call_failure,
    while_loop,
    f64_variable_declaration,
    f64_variable_declaration_and_assignment,
    f64_variable_re_assignment,
    f64_variable_re_assignment_failure,
    i64_variable_declaration,
    i64_variable_declaration_and_assignment,
    i64_variable_re_assignment,
    i64_variable_re_assignment_failure
);
criterion_main!(lang_benches);
