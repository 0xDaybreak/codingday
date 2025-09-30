#[test]
fn test_safe_divide() {
    assert_eq!(safe_divide(10, 2), Ok(5));
    assert_eq!(safe_divide(5, 0), Err("division by zero"));
}