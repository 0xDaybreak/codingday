#[test]
fn test_get_option_value() {
    assert_eq!(get_option_value(Some(42)), 42);
    assert_eq!(get_option_value(None), 0);
}

// 8. Option
pub fn get_option_value(opt: Option<i32>) -> i32 {
    // TODO: return value if Some, else return 0
    unimplemented!()
}
