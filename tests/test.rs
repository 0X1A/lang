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
}
