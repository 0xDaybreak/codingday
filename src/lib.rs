// 🦀 Rust Workshop Exercises
// Each test currently FAILS.
// Your task: make them PASS by writing code!

// 1. Hello Rust!
pub fn hello() -> String {
    // TODO: return "Hello, Rust!"
    unimplemented!()
}


// 2. Mutability
pub fn add_one() {
    // TODO: declare x = 5
    // TODO: return x + 1
    unimplemented!()
}

// 3. Ownership basics
#[derive(Debug)]
struct User {
    id: u32,
}

pub fn print_u() {
    //TODO: fix this compile error
    let u = User{id:1};
    let u2 = u;
    println!("{:?}", u);
    unimplemented!()
}

// 4. Borrowing
pub fn borrow_user() {
    //TODO: print user u twice
    let u = User{id: 123};
    print_user();
    print_user();
    unimplemented!()
}
fn print_user(u: &User) {
    println!("{:?}", u);
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

// 6. Collections (vector)
pub fn sum_vector(v: Vec<i32>) -> i32 {
    // TODO: return the sum of all numbers
    unimplemented!()
}

// 7. Pattern matching
pub enum Direction {
    // TODO: add directions
}

pub fn direction_message(dir: Direction) -> &'static str {
    // TODO: return "Up!", "Down!", etc
    unimplemented!()
}

// 8. Option
pub fn get_option_value(opt: Option<i32>) -> i32 {
    // TODO: return value if Some, else return 0
    unimplemented!()
}

// 9. Result
pub fn safe_divide(a: i32, b: i32) -> Result<i32, &'static str> {
    // TODO: divide, or return Err if b == 0
    unimplemented!()
}

