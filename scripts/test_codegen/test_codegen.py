#!/usr/bin/env python3

import random
import sys
import os
import json
import argparse
from pathlib import Path
from sortedcontainers import SortedDict
from slugify import slugify

""" TODO: Organize this differently, as benchmarks are running these asserts! We don't have anything like run flags to omit asserts """


class LangTestSources(object):
    """
    Contains all sources and methods used for generating tests and benchmarks for the Lang interpreter
    """

    FILE_GENERATION_COMMENT = "// This file is auto-generated. Please do not edit it manually."
    VARIABLE_DECLARATIONS = SortedDict({
        # Comparisons
        # char
        "char equal": """
        let a: char = 'a';
        let b: char = 'a';
        assert(a == b);
        """,
        "char not equal": """
        let a: char = 'a';
        let b: char = 'b';
        assert(a != b);
        """,
        "char greater": """
        let a: i64 = 300;
        let b: i64 = 100;
        assert(a > b);
        """,
        "char greater or equal": """
        let a: char = 'b';
        let b: char = 'b';
        assert(a >= b);
        """,
        "char less": """
        let a: char = 'a';
        let b: char = 'c';
        assert(a < b);
        """,
        "char less or equal": """
        let a: char = 'd';
        let b: char = 'd';
        assert(a <= b);
        """,
        # i64
        "i64 equal": """
        let a: i64 = 100;
        let b: i64 = 100;
        assert(a == b);
        """,
        "i64 not equal": """
        let a: i64 = 300;
        let b: i64 = 100;
        assert(a != b);
        """,
        "i64 greater": """
        let a: i64 = 300;
        let b: i64 = 100;
        assert(a > b);
        """,
        "i64 greater or equal": """
        let a: i64 = 300;
        let b: i64 = 300;
        assert(a >= b);
        """,
        "i64 less": """
        let a: i64 = 300;
        let b: i64 = 500;
        assert(a < b);
        """,
        "i64 less or equal": """
        let a: i64 = 300;
        let b: i64 = 300;
        assert(a <= b);
        """,
        # f64
        "f64 equal": """
        let a: f64 = 100.00;
        let b: f64 = 100.00;
        assert(a == b);
        """,
        "f64 not equal": """
        let a: f64 = 300.00;
        let b: f64 = 100.00;
        assert(a != b);
        """,
        "f64 greater": """
        let a: f64 = 300.00;
        let b: f64 = 100.00;
        assert(a > b);
        """,
        "f64 greater or equal": """
        let a: f64 = 300.00;
        let b: f64 = 300.00;
        assert(a >= b);
        """,
        "f64 less": """
        let a: f64 = 300.00;
        let b: f64 = 500.00;
        assert(a < b);
        """,
        "f64 less or equal": """
        let a: f64 = 300.00;
        let b: f64 = 300.00;
        assert(a <= b);
        """,
        # bool
        "bool equal": """
        let a: bool = false;
        let b: bool = false;
        assert(a == b);
        """,
        "bool not equal": """
        let a: bool = false;
        let b: bool = true;
        assert(a != b);
        """,
        # array
        "array equal": """
        let a: Array<i32> = [0, 1, 2];
        let b: Array<i32> = [0, 1, 2];
        assert(a == b);
        """,
        "array not equal": """
        let a: Array<i32> = [0, 1, 2];
        let b: Array<i32> = [0, 1, 4];
        assert(a != b);
        """,
        # struct vars
        "struct vars equal": """
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
        """,
        "struct vars not equal": """
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
        """,
        # Primitives
        "i64 Variable declaration": """let i: i64;""",
        "i64 Variable declaration and assignment": """let i: i64 = 0;
        assert(i == 0);""",
        "i64 Variable re-assignment": """let i: i64 = 0;
            assert(i == 0);
            i = 100;
            assert(i == 100);""",
        "i64 Variable re-assignment failure":
        """let i: i64 = 0;
        i = 100.00;""",

        "f64 Variable declaration": """let i: f64;""",
        "f64 Variable declaration and assignment": """let i: f64 = 0.00;""",
        "f64 Variable re-assignment": """let i: f64 = 0.00;
            assert(i == 0.00);
            i = 100.00;
            assert(i == 100.00);""",
        "f64 Variable re-assignment failure":
        """let i: f64 = 0;
        i = 100;""",

        # Arrays
        "Array<i64> Variable declaration": """let i: Array<i64>;""",
        "Array<i64> Variable declaration empty": """let i: Array<i64> = [];""",
        "Array<i64> Variable declaration and assignment": """
        let i: Array<i64> = [0, 1, 2];
        assert(i[0] == 0);
        assert(i[1] == 1);
        assert(i[2] == 2);
        """,
        "Array<i64> Variable re-assignment": """
        let i: Array<i64> = [];
        i = [0, 1, 2];
        assert(i[0] == 0);
        assert(i[1] == 1);
        assert(i[2] == 2);
        """,
        "Array<i64> Variable re-assignment failure": """let i: Array<i64> = [];
            i = [0.00, 1.00, 2.00];
        assert(i[0] == 0.00);
        assert(i[1] == 1.00);
        assert(i[2] == 2.00);
            """,

        # Struct
        "Struct declaration": """struct TestStruct {}""",
        "Struct declaration failure": """struct TestStruct {{}""",
        "Struct declaration with fields": """
        struct TestStruct {
            field0: i64,
            field1: f64,
            field2: bool
        }
        """,
        "Struct with fields instatiation": """
        struct TestStruct {
            field0: i64,
            field1: f64,
            field2: bool
        }

        let instance: TestStruct = TestStruct();
        """,
        "Struct with fields instatiation and field assignment": """
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
        """,
        "Struct with field access failure": """
        struct TestStruct {}

        let instance: TestStruct = TestStruct();
        instance.field_doesnt_exist;
        """,
        "Struct with impl": """
        struct TestStruct {
        }

        impl TestStruct {
            fn hello() -> () {
                print \\"Hello world\\";
            }
        }

        let instance: TestStruct = TestStruct();
        instance.hello();
        """,
        "Struct with impl using self": """
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
        """,
        "Struct with impl using mutable self": """
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
        """,
        "Struct with method call failure": """
        struct TestStruct {
        }

        impl TestStruct {
        }

        let instance: TestStruct = TestStruct();
        instance.hello();
        """,
        # Control Flow
        "For loop": """
        for (let i: i32 = 0; i < 10; i = i + 1) {
            print i;
        }
        for (let b: bool = false; b == true; b = false) {
            print b;
        }
        """,
        "While Loop": """
        let b: bool = true;
        let i: i32 = 0;
        while (b) {
            i = i + 1;
            if (i == 10) {
                b = false;
            }
        }
        assert(b == false);
        """,
        "Return from block": """
        fn test() -> i32 {
            {
                return 100;
            }
        }
        let value: i32 = test();
        assert(value == 100);
        """,
        "Assertion failure": """
        assert(0 == 100);
        """,
        "Assertion": """
        assert(100 == 100);
        assert(true == true);
        assert(1.05 == 1.05);
        """,
        # Functions
        "Function returns bool": """
        fn test() -> bool {
            return false;
        }
        assert(false == test());
        """,
        "Function returns bool with args": """
        fn test(a: i32, b: f64) -> bool {
            return false;
        }
        assert(false == test(100, 100.00));
        """,
        "Function assert failure": """
        fn test(a: i32, b: f64) -> bool {
            if (!(a > 1000)) {
                assert(false);
            }
            return false;
        }
        test(1, 0.0);
        """,
        "Function nested return": """
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
        """,
    })

    def generate_files(self, path):
        output_dir = Path(path)
        if not output_dir.exists():
            print("Creating directory '{}'".format(output_dir))
            os.mkdir(output_dir)
        for key, value in self.VARIABLE_DECLARATIONS.items():
            source_file_path = output_dir / \
                (slugify(key, separator="_") + ".lang")
            with open(source_file_path, 'w') as source_file:
                source_file.write(self.FILE_GENERATION_COMMENT + '\n')
                source_file.write(value)
                print("Wrote file '{}'".format(source_file_path))

    def print_lang_benches(self):
        """Prints benches for running at the top-most interface of the interpreter"""
        print(self.FILE_GENERATION_COMMENT)
        print("""
        #[macro_use]
        extern crate criterion;
        extern crate lang;

        use criterion::{black_box, Criterion};
        use self::lang::lang::Lang;
        """)
        slugged_keys = []
        for key, value in self.VARIABLE_DECLARATIONS.items():
            key_slug = slugify(key, separator="_")
            slugged_keys.append(key_slug)
            print("fn {}(c: &mut Criterion) {{".format(
                key_slug))
            print("c.bench_function(\"{}\", |b| {{".format(key))
            print("b.iter(|| Lang::new(black_box(Some(\"{}\"))))".format(value))
            print("});}")
        criterion_string = ", ".join(slugged_keys)
        print("criterion_group!(lang_benches, {});".format(criterion_string))
        print("criterion_main!(lang_benches);")

    def print_tests(self):
        source = """
        #[cfg(test)]
        mod tests {
        extern crate lang;
        use self::lang::lang::Lang;
        """
        print(self.FILE_GENERATION_COMMENT)
        print(source)
        for key, value in self.VARIABLE_DECLARATIONS.items():
            should_fail = key.endswith("failure")
            print("#[test]")
            print("fn {}() {{".format(slugify(key, separator="_")))
            print("let mut lang = Lang::new(Some(\"{}\"));".format(value))
            print("let result = lang.run();")
            print(
                "if let Err(ref error) = result { println!(\"{}\", error); }")
            print("assert_eq!(result.is_ok(), {}) }}".format(
                "false" if should_fail else "true"))
        print("}")


def main(argv):
    LANG_TEST_SOURCES = LangTestSources()
    arg_parser = argparse.ArgumentParser()
    arg_parser.add_argument("--print_tests", action='store_true')
    arg_parser.add_argument("--print_lang_benches", action='store_true')
    arg_parser.add_argument("--generate_files", action='store_true')

    args = arg_parser.parse_args()

    if args.print_tests:
        LANG_TEST_SOURCES.print_tests()
    if args.print_lang_benches:
        LANG_TEST_SOURCES.print_lang_benches()
    if args.generate_files:
        LANG_TEST_SOURCES.generate_files("scripts")


if __name__ == "__main__":
    main(sys.argv)
