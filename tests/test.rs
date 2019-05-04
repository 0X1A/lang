#[cfg(test)]
mod tests {

    extern crate lang;

    use self::lang::lang::Lang;
    // Test variable declaration
    #[test]
    fn variable_empty_decl() {
        let mut lang = Lang::new(Some("let variable: i32;"));
        let result = lang.run().is_ok();
        assert_eq!(result, true);
    }

    // Test variable declaration
    #[test]
    fn variable_decl() {
        let mut lang = Lang::new(Some("let variable: i64 = 100; print variable;"));
        let result = lang.run().is_ok();
        assert_eq!(result, true);
    }

    // Test variable declaration
    #[test]
    fn variable_i32_type_assignment_failure() {
        let mut lang = Lang::new(Some("let variable: i32 = false;"));
        let result = lang.run().is_ok();
        assert_eq!(result, false);
    }

    // Test variable declaration
    #[test]
    fn variable_f32_type_assignment_failure() {
        let mut lang = Lang::new(Some("let variable: f32 = \"Hello World\";"));
        let result = lang.run().is_ok();
        assert_eq!(result, false);
    }

    // Test variable declaration
    #[test]
    fn variable_string_type_assignment_failure() {
        let mut lang = Lang::new(Some("let variable: String = 100;"));
        let result = lang.run().is_ok();
        assert_eq!(result, false);
    }

    // Test variable declaration
    #[test]
    fn struct_decl() {
        let mut lang = Lang::new(Some("struct TestStruct {}"));
        let result = lang.run().is_ok();
        assert_eq!(result, true);
    }

    // Test variable declaration
    #[test]
    fn struct_decl_with_fields() {
        let mut lang = Lang::new(Some("struct TestStruct { one: i32, two: String }"));
        let result = lang.run().is_ok();
        assert_eq!(result, true);
    }

    // Test variable declaration
    #[test]
    fn struct_decl_with_fields_failure() {
        let mut lang = Lang::new(Some("struct TestStruct { one: two: String }"));
        let result = lang.run().is_ok();
        assert_eq!(result, false);
    }

    // Test variable declaration
    #[test]
    fn struct_decl_with_impl() {
        let mut lang = Lang::new(Some("struct TestStruct { one: i32 } impl TestStruct { }"));
        let result = lang.run().is_ok();
        assert_eq!(result, true);
    }

    // Test variable declaration
    #[test]
    fn array_index() {
        let mut lang = Lang::new(Some(
            "let arr: Array<i64> = [0, 1, 2]; let idx: i64 = arr[0];",
        ));
        let result = lang.run().is_ok();
        assert_eq!(result, true);
    }
}
