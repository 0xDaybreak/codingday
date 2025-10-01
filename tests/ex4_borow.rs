#[test]
fn test_borrow_user() {
    borrow_user();
}

// 4. Borrowing
pub fn borrow_user() {
    //TODO: print user u twice
    let u = User { id: 123 };
    print_user();
    print_user();
    unimplemented!()
}
fn print_user(u: ) {
    println!("{:?}", u);
}
