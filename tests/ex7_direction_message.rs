use tests::{direction_message, Direction};

#[test]
fn test_direction_message() {
    assert_eq!(direction_message(Direction::Up), "Up!");
    assert_eq!(direction_message(Direction::Left), "Left!");
}
