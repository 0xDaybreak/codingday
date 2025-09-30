use tests::get_option_value;

#[test]
fn test_get_option_value() {
    assert_eq!(get_option_value(Some(42)), 42);
    assert_eq!(get_option_value(None), 0);
}
