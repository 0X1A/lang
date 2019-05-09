#!/usr/bin/env python3

import random
import sys
import os
import json
import argparse
from pathlib import Path
from slugify import slugify


class LangTestSources(object):
    """
    Contains all sources and methods used for generating tests and benchmarks for the Lang interpreter
    """

    FILE_GENERATION_COMMENT = "// This file is auto-generated. Please do not edit it manually."
    VARIABLE_DECLARATIONS = {
        # Primitives
        "i64 Variable declaration": """let i: i64;""",
        "i64 Variable declaration and assignment": """let i: i64 = 0;""",
        "i64 Variable re-assignment": """let i: i64 = 0;
            i = 100;""",
        "i64 Variable re-assignment failure":
        """let i: i64 = 0;
        i = 100;""",

        "f64 Variable declaration": """let i: f64;""",
        "f64 Variable declaration and assignment": """let i: f64 = 0.00;""",
        "f64 Variable re-assignment": """let i: f64 = 0.00;
            i = 100.00;""",
        "f64 Variable re-assignment failure":
        """let i: f64 = 0;
        i = 100;""",

        # Arrays
        "Array<i64> Variable declaration": """let i: Array<i64>;""",
        "Array<i64> Variable declaration empty": """let i: Array<i64> = [];""",
        "Array<i64> Variable declaration and assignment": """let i: Array<i64> = [0, 1, 2];""",
        "Array<i64> Variable re-assignment": """let i: Array<i64> = [];
            i = [0, 1, 2];""",
        "Array<i64> Variable re-assignment failure": """let i: Array<i64> = [];
            i = [0.00, 1.00, 2.00];""",

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
        "Struct with method call failure": """
        struct TestStruct {
        }

        impl TestStruct {
        }

        let instance: TestStruct = TestStruct();
        instance.hello();
        """,
    }

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
