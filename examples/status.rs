extern crate rider;

use rider::libs::state::state::{Status};
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Debug)]
struct Point {
    x: f64,
    y: f64,
}


/// Implementation block, all `Point` methods go in here
// メソッドの実装のためのブロック。`Point`の持つメソッドを全て定義する。
impl Point {
    // This is a static method
    // Static methods don't need to be called by an instance
    // These methods are generally used as constructors
    // スタティックメソッド。つまり、インスタンスからでなくても
    // 呼び出せるメソッド。以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another static method, taking two arguments:
    // もう一つスタティックメソッド。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is an instance method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはインスタンスメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }
}

fn call(number: &str) -> &str {
    match number {
        "798-1364" => "We're sorry, the call cannot be completed as dialed. 
            Please hang up and try again.",
        "645-7689" => "Hello, this is Mr. Awesome's Pizza. My name is Fred.
            What can I get for you today?",
        _ => "Hi! Who is this again?"
    }
}

fn main() {
    let point = Point{x: 23.0, y: 32.0};
    let two_point = Point::new(30.2, 20.1);
    let rectangle = Rectangle{p1: point, p2: two_point};

    let mut contacts = HashMap::new();
    let mut rectangle_hash = HashMap::new();
    let mut test_hash = HashMap::new();
    contacts.insert("Daniel", two_point);
    contacts.insert("Ashley", point);
    rectangle_hash.insert("rectangle1", rectangle);
    test_hash.insert("test1", &contacts);
    // Takes a reference and returns Option<&V>
    // 参照をとり、Option<&V>を返す。
    match contacts.get(&"Daniel") {
        Some(&number) => println!(" test Calling Daniel: {:?}", number),
        _ => println!("Don't have Daniel's number."),
    }
    // println!("status: {:?}", status);
    println!("two_point: {:?}", two_point);
    println!("rectangle: {:?}", rectangle);
    println!("{:?}", rectangle.area());
}