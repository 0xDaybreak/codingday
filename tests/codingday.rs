use tests::*;
#[test]
fn test_hello() {
    assert_eq!(hello(), "Hello, Rust!");
}

#[test]
fn test_add_one() {
    assert_eq!(add_one(5), 6);
}

#[test]
fn test_move_string() {
    let s = move_string();
    assert_eq!(s, "hello");
}

#[test]
fn test_borrow_string() {
    let s = String::from("borrow");
    assert_eq!(borrow_string(&s), 6);
}

#[test]
fn test_print_and_return() {
    let s = String::from("Rust");
    let r = print_and_return(s);
    assert_eq!(r, "Rust");
}

#[test]
fn test_rectangle_area() {
    let rect = Rectangle { width: 10, height: 5 };
    assert_eq!(rect.area(), 50);
}

#[test]
fn test_sum_vector() {
    assert_eq!(sum_vector(vec![1, 2, 3, 4]), 10);
}

#[test]
fn test_direction_message() {
    assert_eq!(direction_message(Direction::Up), "Up!");
    assert_eq!(direction_message(Direction::Left), "Left!");
}

#[test]
fn test_get_option_value() {
    assert_eq!(get_option_value(Some(42)), 42);
    assert_eq!(get_option_value(None), 0);
}

#[test]
fn test_safe_divide() {
    assert_eq!(safe_divide(10, 2), Ok(5));
    assert_eq!(safe_divide(5, 0), Err("division by zero"));
}
