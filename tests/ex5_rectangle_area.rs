use tests::*;

#[test]
fn test_rectangle_area() {
    let rect = Rectangle { width: 10, height: 5 };
    assert_eq!(rect.area(), 50);
}