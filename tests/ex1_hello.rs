#[test]
fn test_hello() {
    assert_eq!(hello(), "Hello, Rust!");
}


// 1. Hello Rust!
pub fn hello() -> String {
    // TODO: return "Hello, Rust!"
    return "Hello, Rust!".to_string();
}
