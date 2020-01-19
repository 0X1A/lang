// This file is auto-generated. Please do not edit it manually.

#[cfg(test)]
mod tests {
    extern crate lang;
    use self::lang::lang::Lang;

    #[test]
    fn array_i64_variable_declaration() {
        let mut lang = Lang::new(Some("let i: Array<i64>;"));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn array_i64_variable_declaration_and_assignment() {
        let mut lang = Lang::new(Some(
            "
        let i: Array<i64> = [0, 1, 2];
        assert(i[0] == 0);
        assert(i[1] == 1);
        assert(i[2] == 2);
        ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn array_i64_variable_declaration_empty() {
        let mut lang = Lang::new(Some("let i: Array<i64> = [];"));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn array_i64_variable_re_assignment() {
        let mut lang = Lang::new(Some(
            "
        let i: Array<i64> = [];
        i = [0, 1, 2];
        assert(i[0] == 0);
        assert(i[1] == 1);
        assert(i[2] == 2);
        ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn array_i64_variable_re_assignment_failure() {
        let mut lang = Lang::new(Some(
            "let i: Array<i64> = [];
            i = [0.00, 1.00, 2.00];
        assert(i[0] == 0.00);
        assert(i[1] == 1.00);
        assert(i[2] == 2.00);
            ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), false)
    }
    #[test]
    fn assertion() {
        let mut lang = Lang::new(Some(
            "
        assert(100 == 100);
        assert(true == true);
        assert(1.05 == 1.05);
        ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn assertion_failure() {
        let mut lang = Lang::new(Some(
            "
        assert(0 == 100);
        ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), false)
    }
    #[test]
    fn for_loop() {
        let mut lang = Lang::new(Some(
            "
        for (let i: i32 = 0; i < 10; i = i + 1) {
            print i;
        }
        for (let b: bool = false; b == true; b = false) {
            print b;
        }
        ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn return_from_block() {
        let mut lang = Lang::new(Some(
            "
        fn test() -> i32 {
            {
                return 100;
            }
        }
        let value: i32 = test();
        assert(value == 100);
        ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn struct_declaration() {
        let mut lang = Lang::new(Some("struct TestStruct {}"));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn struct_declaration_failure() {
        let mut lang = Lang::new(Some("struct TestStruct {{}"));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), false)
    }
    #[test]
    fn struct_declaration_with_fields() {
        let mut lang = Lang::new(Some(
            "
        struct TestStruct {
            field0: i64,
            field1: f64,
            field2: bool
        }
        ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn struct_with_field_access_failure() {
        let mut lang = Lang::new(Some(
            "
        struct TestStruct {}

        let instance: TestStruct = TestStruct();
        instance.field_doesnt_exist;
        ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), false)
    }
    #[test]
    fn struct_with_fields_instatiation() {
        let mut lang = Lang::new(Some(
            "
        struct TestStruct {
            field0: i64,
            field1: f64,
            field2: bool
        }

        let instance: TestStruct = TestStruct();
        ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn struct_with_fields_instatiation_and_field_assignment() {
        let mut lang = Lang::new(Some(
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
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn struct_with_impl() {
        let mut lang = Lang::new(Some(
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
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn struct_with_impl_using_mutable_self() {
        let mut lang = Lang::new(Some(
            "
        struct TestStruct {
            i: i32,
        }

        impl TestStruct {
            fn hello(other: TestStruct) -> () {
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
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn struct_with_impl_using_self() {
        let mut lang = Lang::new(Some(
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
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn struct_with_method_call_failure() {
        let mut lang = Lang::new(Some(
            "
        struct TestStruct {
        }

        impl TestStruct {
        }

        let instance: TestStruct = TestStruct();
        instance.hello();
        ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), false)
    }
    #[test]
    fn while_loop() {
        let mut lang = Lang::new(Some(
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
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn array_equal() {
        let mut lang = Lang::new(Some(
            "
        let a: Array<i32> = [0, 1, 2];
        let b: Array<i32> = [0, 1, 2];
        assert(a == b);
        ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn array_not_equal() {
        let mut lang = Lang::new(Some(
            "
        let a: Array<i32> = [0, 1, 2];
        let b: Array<i32> = [0, 1, 4];
        assert(a != b);
        ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn bool_equal() {
        let mut lang = Lang::new(Some(
            "
        let a: bool = false;
        let b: bool = false;
        assert(a == b);
        ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn bool_not_equal() {
        let mut lang = Lang::new(Some(
            "
        let a: bool = false;
        let b: bool = true;
        assert(a != b);
        ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn char_equal() {
        let mut lang = Lang::new(Some(
            "
        let a: char = 'a';
        let b: char = 'a';
        assert(a == b);
        ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn char_greater() {
        let mut lang = Lang::new(Some(
            "
        let a: i64 = 300;
        let b: i64 = 100;
        assert(a > b);
        ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn char_greater_or_equal() {
        let mut lang = Lang::new(Some(
            "
        let a: char = 'b';
        let b: char = 'b';
        assert(a >= b);
        ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn char_less() {
        let mut lang = Lang::new(Some(
            "
        let a: char = 'a';
        let b: char = 'c';
        assert(a < b);
        ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn char_less_or_equal() {
        let mut lang = Lang::new(Some(
            "
        let a: char = 'd';
        let b: char = 'd';
        assert(a <= b);
        ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn char_not_equal() {
        let mut lang = Lang::new(Some(
            "
        let a: char = 'a';
        let b: char = 'b';
        assert(a != b);
        ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn f64_variable_declaration() {
        let mut lang = Lang::new(Some("let i: f64;"));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn f64_variable_declaration_and_assignment() {
        let mut lang = Lang::new(Some("let i: f64 = 0.00;"));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn f64_variable_re_assignment() {
        let mut lang = Lang::new(Some(
            "let i: f64 = 0.00;
            assert(i == 0.00);
            i = 100.00;
            assert(i == 100.00);",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn f64_variable_re_assignment_failure() {
        let mut lang = Lang::new(Some(
            "let i: f64 = 0;
        i = 100;",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), false)
    }
    #[test]
    fn f64_equal() {
        let mut lang = Lang::new(Some(
            "
        let a: f64 = 100.00;
        let b: f64 = 100.00;
        assert(a == b);
        ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn f64_greater() {
        let mut lang = Lang::new(Some(
            "
        let a: f64 = 300.00;
        let b: f64 = 100.00;
        assert(a > b);
        ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn f64_greater_or_equal() {
        let mut lang = Lang::new(Some(
            "
        let a: f64 = 300.00;
        let b: f64 = 300.00;
        assert(a >= b);
        ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn f64_less() {
        let mut lang = Lang::new(Some(
            "
        let a: f64 = 300.00;
        let b: f64 = 500.00;
        assert(a < b);
        ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn f64_less_or_equal() {
        let mut lang = Lang::new(Some(
            "
        let a: f64 = 300.00;
        let b: f64 = 300.00;
        assert(a <= b);
        ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn f64_not_equal() {
        let mut lang = Lang::new(Some(
            "
        let a: f64 = 300.00;
        let b: f64 = 100.00;
        assert(a != b);
        ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn i64_variable_declaration() {
        let mut lang = Lang::new(Some("let i: i64;"));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn i64_variable_declaration_and_assignment() {
        let mut lang = Lang::new(Some(
            "let i: i64 = 0;
        assert(i == 0);",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn i64_variable_re_assignment() {
        let mut lang = Lang::new(Some(
            "let i: i64 = 0;
            assert(i == 0);
            i = 100;
            assert(i == 100);",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn i64_variable_re_assignment_failure() {
        let mut lang = Lang::new(Some(
            "let i: i64 = 0;
        i = 100.00;",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), false)
    }
    #[test]
    fn i64_equal() {
        let mut lang = Lang::new(Some(
            "
        let a: i64 = 100;
        let b: i64 = 100;
        assert(a == b);
        ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn i64_greater() {
        let mut lang = Lang::new(Some(
            "
        let a: i64 = 300;
        let b: i64 = 100;
        assert(a > b);
        ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn i64_greater_or_equal() {
        let mut lang = Lang::new(Some(
            "
        let a: i64 = 300;
        let b: i64 = 300;
        assert(a >= b);
        ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn i64_less() {
        let mut lang = Lang::new(Some(
            "
        let a: i64 = 300;
        let b: i64 = 500;
        assert(a < b);
        ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn i64_less_or_equal() {
        let mut lang = Lang::new(Some(
            "
        let a: i64 = 300;
        let b: i64 = 300;
        assert(a <= b);
        ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn i64_not_equal() {
        let mut lang = Lang::new(Some(
            "
        let a: i64 = 300;
        let b: i64 = 100;
        assert(a != b);
        ",
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn struct_vars_equal() {
        let mut lang = Lang::new(Some(
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
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
    #[test]
    fn struct_vars_not_equal() {
        let mut lang = Lang::new(Some(
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
        ));
        let result = lang.run();
        if let Err(ref error) = result {
            println!("{}", error);
        }
        assert_eq!(result.is_ok(), true)
    }
}
