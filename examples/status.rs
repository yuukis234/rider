extern crate rider;

use rider::libs::state::state::{Status};
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32
}



fn main() {
    let point = Point{x: 23, y: 32};
    let status = Status{state: point};
    let mut hash = HashMap::new();
    hash.insert("apple".to_owned(), 1);
    hash.insert("orange".to_owned(), 2);
    hash.insert("coffee".to_owned(), 42);
    println!("status: {:?}", status);
    println!("hash: {:?}", hash);
}