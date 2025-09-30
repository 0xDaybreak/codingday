// 🦀 Rust Workshop Exercises
// Each test currently FAILS.
// Your task: make them PASS by writing code!

// 1. Hello Rust!
pub fn hello() -> String {
    "Hello, Rust!".to_string()
}

// 2. Mutability
pub fn add_one(mut x: i32) -> i32 {
    // TODO: increase x by 1
    unimplemented!()
}

// 3. Ownership basics
pub fn move_string() -> String {
    let s1 = String::from("hello");
    // TODO: return s1 without causing a move error
    unimplemented!()
}

// 4. Borrowing
pub fn borrow_string(s: &String) -> usize {
    // TODO: return the length of s
    unimplemented!()
}

// 5. Functions & ownership
pub fn print_and_return(s: String) -> String {
    // TODO: print s, then return it so caller still owns it
    unimplemented!()
}

// 6. Structs & methods
pub struct Rectangle {
    // TODO: add width and height fields
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        // TODO: compute area
        unimplemented!()
    }
}

// 7. Collections (vector)
pub fn sum_vector(v: Vec<i32>) -> i32 {
    // TODO: return the sum of all numbers
    unimplemented!()
}

// 8. Pattern matching
pub enum Direction {
    // TODO: Up, Down, Left, Right
}

pub fn direction_message(dir: Direction) -> &'static str {
    // TODO: return "Up!", "Down!", etc
    unimplemented!()
}

// 9. Option & Result
pub fn get_option_value(opt: Option<i32>) -> i32 {
    // TODO: return value if Some, else return 0
    unimplemented!()
}

pub fn safe_divide(a: i32, b: i32) -> Result<i32, &'static str> {
    // TODO: divide, or return Err if b == 0
    unimplemented!()
}

