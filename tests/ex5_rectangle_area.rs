#[test]
fn test_rectangle_area() {
    let rect = Rectangle { width: 10, height: 5 };
    assert_eq!(rect.area(), 50);
}

// 5. Structs & methods
pub struct Rectangle {
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        // TODO: compute area
        unimplemented!()
    }
}
