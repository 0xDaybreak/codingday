#[test]
fn test_safe_divide() {
    assert_eq!(safe_divide(10, 2), Ok(5));
    assert_eq!(safe_divide(5, 0), Err("division by zero"));
}

// 9. Result
pub fn safe_divide(a: i32, b: i32) -> Result<i32, &'static str> {
    // TODO: divide, or return Err if b == 0
    unimplemented!()
}
