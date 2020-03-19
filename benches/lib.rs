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
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        let i: Array<i64> = [0, 1, 2];
        assert(i[0] == 0);
        assert(i[1] == 1);
        assert(i[2] == 2);
        ",
            )))
        })
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
                "
        let i: Array<i64> = [];
        i = [0, 1, 2];
        assert(i[0] == 0);
        assert(i[1] == 1);
        assert(i[2] == 2);
        ",
            )))
        })
    });
}
fn array_i64_variable_re_assignment_failure(c: &mut Criterion) {
    c.bench_function("Array<i64> Variable re-assignment failure", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "let i: Array<i64> = [];
            i = [0.00, 1.00, 2.00];
        assert(i[0] == 0.00);
        assert(i[1] == 1.00);
        assert(i[2] == 2.00);
            ",
            )))
        })
    });
}
fn assertion(c: &mut Criterion) {
    c.bench_function("Assertion", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        assert(100 == 100);
        assert(true == true);
        assert(1.05 == 1.05);
        ",
            )))
        })
    });
}
fn assertion_failure(c: &mut Criterion) {
    c.bench_function("Assertion failure", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        assert(0 == 100);
        ",
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
fn function_assert_failure(c: &mut Criterion) {
    c.bench_function("Function assert failure", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        fn test(a: i32, b: f64) -> bool {
            if (!(a > 1000)) {
                assert(false);
            }
            return false;
        }
        test(1, 0.0);
        ",
            )))
        })
    });
}
fn function_nested_return(c: &mut Criterion) {
    c.bench_function("Function nested return", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        fn test() -> bool {
            {
                {
                    {
                        return false;
                    }
                }
            }
        }
        assert(false == test());
        ",
            )))
        })
    });
}
fn function_returns_bool(c: &mut Criterion) {
    c.bench_function("Function returns bool", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        fn test() -> bool {
            return false;
        }
        assert(false == test());
        ",
            )))
        })
    });
}
fn function_returns_bool_with_args(c: &mut Criterion) {
    c.bench_function("Function returns bool with args", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        fn test(a: i32, b: f64) -> bool {
            return false;
        }
        assert(false == test(100, 100.00));
        ",
            )))
        })
    });
}
fn return_from_block(c: &mut Criterion) {
    c.bench_function("Return from block", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        fn test() -> i32 {
            {
                return 100;
            }
        }
        let value: i32 = test();
        assert(value == 100);
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
        assert(instance.field0 == 0);
        assert(instance.field1 == 1.00);
        assert(instance.field2 == false);
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
fn struct_with_impl_using_mutable_self(c: &mut Criterion) {
    c.bench_function("Struct with impl using mutable self", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        struct TestStruct {
            i: i32,
        }

        impl TestStruct {
            fn hello(self: TestStruct, other: TestStruct) -> () {
                self.i = self.i + other.i;
            }
        }

        let instance: TestStruct = TestStruct();
        let other: TestStruct = TestStruct();
        other.i = 100;
        instance.i = 100;
        instance.hello(other);
        assert(instance.i == 200);
        ",
            )))
        })
    });
}
fn struct_with_impl_using_self(c: &mut Criterion) {
    c.bench_function("Struct with impl using self", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        struct TestStruct {
            i: i32,
        }

        impl TestStruct {
            fn hello() -> () {
                print self.i;
            }
        }

        let instance: TestStruct = TestStruct();
        instance.i = 100;
        assert(instance.i == 100);
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
        assert(b == false);
        ",
            )))
        })
    });
}
fn array_equal(c: &mut Criterion) {
    c.bench_function("array equal", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        let a: Array<i32> = [0, 1, 2];
        let b: Array<i32> = [0, 1, 2];
        assert(a == b);
        ",
            )))
        })
    });
}
fn array_not_equal(c: &mut Criterion) {
    c.bench_function("array not equal", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        let a: Array<i32> = [0, 1, 2];
        let b: Array<i32> = [0, 1, 4];
        assert(a != b);
        ",
            )))
        })
    });
}
fn bool_equal(c: &mut Criterion) {
    c.bench_function("bool equal", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        let a: bool = false;
        let b: bool = false;
        assert(a == b);
        ",
            )))
        })
    });
}
fn bool_not_equal(c: &mut Criterion) {
    c.bench_function("bool not equal", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        let a: bool = false;
        let b: bool = true;
        assert(a != b);
        ",
            )))
        })
    });
}
fn char_equal(c: &mut Criterion) {
    c.bench_function("char equal", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        let a: char = 'a';
        let b: char = 'a';
        assert(a == b);
        ",
            )))
        })
    });
}
fn char_greater(c: &mut Criterion) {
    c.bench_function("char greater", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        let a: i64 = 300;
        let b: i64 = 100;
        assert(a > b);
        ",
            )))
        })
    });
}
fn char_greater_or_equal(c: &mut Criterion) {
    c.bench_function("char greater or equal", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        let a: char = 'b';
        let b: char = 'b';
        assert(a >= b);
        ",
            )))
        })
    });
}
fn char_less(c: &mut Criterion) {
    c.bench_function("char less", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        let a: char = 'a';
        let b: char = 'c';
        assert(a < b);
        ",
            )))
        })
    });
}
fn char_less_or_equal(c: &mut Criterion) {
    c.bench_function("char less or equal", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        let a: char = 'd';
        let b: char = 'd';
        assert(a <= b);
        ",
            )))
        })
    });
}
fn char_not_equal(c: &mut Criterion) {
    c.bench_function("char not equal", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        let a: char = 'a';
        let b: char = 'b';
        assert(a != b);
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
            assert(i == 0.00);
            i = 100.00;
            assert(i == 100.00);",
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
fn f64_equal(c: &mut Criterion) {
    c.bench_function("f64 equal", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        let a: f64 = 100.00;
        let b: f64 = 100.00;
        assert(a == b);
        ",
            )))
        })
    });
}
fn f64_greater(c: &mut Criterion) {
    c.bench_function("f64 greater", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        let a: f64 = 300.00;
        let b: f64 = 100.00;
        assert(a > b);
        ",
            )))
        })
    });
}
fn f64_greater_or_equal(c: &mut Criterion) {
    c.bench_function("f64 greater or equal", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        let a: f64 = 300.00;
        let b: f64 = 300.00;
        assert(a >= b);
        ",
            )))
        })
    });
}
fn f64_less(c: &mut Criterion) {
    c.bench_function("f64 less", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        let a: f64 = 300.00;
        let b: f64 = 500.00;
        assert(a < b);
        ",
            )))
        })
    });
}
fn f64_less_or_equal(c: &mut Criterion) {
    c.bench_function("f64 less or equal", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        let a: f64 = 300.00;
        let b: f64 = 300.00;
        assert(a <= b);
        ",
            )))
        })
    });
}
fn f64_not_equal(c: &mut Criterion) {
    c.bench_function("f64 not equal", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        let a: f64 = 300.00;
        let b: f64 = 100.00;
        assert(a != b);
        ",
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
        b.iter(|| {
            Lang::new(black_box(Some(
                "let i: i64 = 0;
        assert(i == 0);",
            )))
        })
    });
}
fn i64_variable_re_assignment(c: &mut Criterion) {
    c.bench_function("i64 Variable re-assignment", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "let i: i64 = 0;
            assert(i == 0);
            i = 100;
            assert(i == 100);",
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
fn i64_equal(c: &mut Criterion) {
    c.bench_function("i64 equal", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        let a: i64 = 100;
        let b: i64 = 100;
        assert(a == b);
        ",
            )))
        })
    });
}
fn i64_greater(c: &mut Criterion) {
    c.bench_function("i64 greater", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        let a: i64 = 300;
        let b: i64 = 100;
        assert(a > b);
        ",
            )))
        })
    });
}
fn i64_greater_or_equal(c: &mut Criterion) {
    c.bench_function("i64 greater or equal", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        let a: i64 = 300;
        let b: i64 = 300;
        assert(a >= b);
        ",
            )))
        })
    });
}
fn i64_less(c: &mut Criterion) {
    c.bench_function("i64 less", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        let a: i64 = 300;
        let b: i64 = 500;
        assert(a < b);
        ",
            )))
        })
    });
}
fn i64_less_or_equal(c: &mut Criterion) {
    c.bench_function("i64 less or equal", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        let a: i64 = 300;
        let b: i64 = 300;
        assert(a <= b);
        ",
            )))
        })
    });
}
fn i64_not_equal(c: &mut Criterion) {
    c.bench_function("i64 not equal", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        let a: i64 = 300;
        let b: i64 = 100;
        assert(a != b);
        ",
            )))
        })
    });
}
fn struct_vars_equal(c: &mut Criterion) {
    c.bench_function("struct vars equal", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        struct Test {
            a: i32,
            b: bool,
            c: f64,
        }
        let instance: Test = Test();
        instance.a = 100;
        instance.b = true;
        instance.c = 10.05;
        assert(instance.a == 100);
        assert(instance.b == true);
        ",
            )))
        })
    });
}
fn struct_vars_not_equal(c: &mut Criterion) {
    c.bench_function("struct vars not equal", |b| {
        b.iter(|| {
            Lang::new(black_box(Some(
                "
        struct Test {
            a: i32,
            b: bool,
            c: f64,
        }
        let instance: Test = Test();
        instance.a = 100;
        instance.b = true;
        instance.c = 10.05;
        assert(instance.a != 101);
        assert(instance.b != false);
        ",
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
    assertion,
    assertion_failure,
    for_loop,
    function_assert_failure,
    function_nested_return,
    function_returns_bool,
    function_returns_bool_with_args,
    return_from_block,
    struct_declaration,
    struct_declaration_failure,
    struct_declaration_with_fields,
    struct_with_field_access_failure,
    struct_with_fields_instatiation,
    struct_with_fields_instatiation_and_field_assignment,
    struct_with_impl,
    struct_with_impl_using_mutable_self,
    struct_with_impl_using_self,
    struct_with_method_call_failure,
    while_loop,
    array_equal,
    array_not_equal,
    bool_equal,
    bool_not_equal,
    char_equal,
    char_greater,
    char_greater_or_equal,
    char_less,
    char_less_or_equal,
    char_not_equal,
    f64_variable_declaration,
    f64_variable_declaration_and_assignment,
    f64_variable_re_assignment,
    f64_variable_re_assignment_failure,
    f64_equal,
    f64_greater,
    f64_greater_or_equal,
    f64_less,
    f64_less_or_equal,
    f64_not_equal,
    i64_variable_declaration,
    i64_variable_declaration_and_assignment,
    i64_variable_re_assignment,
    i64_variable_re_assignment_failure,
    i64_equal,
    i64_greater,
    i64_greater_or_equal,
    i64_less,
    i64_less_or_equal,
    i64_not_equal,
    struct_vars_equal,
    struct_vars_not_equal
);
criterion_main!(lang_benches);
