#[test]
fn test_move_string() {
    print_u();
}

// 3. Ownership basics
#[derive(Debug)]
struct User {
    id: u32,
}

pub fn print_u() {
    //TODO: fix this compile error
    let u = User { id: 1 };
    let u2 = u;
    println!("{:?}", u);
    unimplemented!()
}
