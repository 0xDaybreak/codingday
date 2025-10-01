#[test]
fn test_direction_message() {
    assert_eq!(direction_message(Direction::Up), "Up!");
    assert_eq!(direction_message(Direction::Left), "Left!");
}


// 7. Pattern matching
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn direction_message(dir: Direction) -> &'static str {
    // TODO: return "Up!", "Down!", etc
    unimplemented!()
}
}