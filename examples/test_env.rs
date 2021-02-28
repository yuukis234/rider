use std::fmt::Debug;
use std::rc::Rc;

use rider::libs::state::state::*;

trait Animal {
    fn talk(&self) {
        println!("souzou ");
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Status <'a> {
    pub hash: &'a str,
    pub state: f64,
}

impl Status <'_> {
    pub fn test(&self) -> f64 {
        return 0.1
    }

    pub fn getkey(&self) -> f64 {
        return self.state;
    }

    pub fn gethash(&self) -> &str {
        return self.hash;
    }
}

impl Animal for Status <'_> {
}

struct TestStruct {
}

impl Animal for TestStruct {}

fn test_func<T: Animal>(arg: &T) {
    arg.talk();
}


struct QuitMessage; // ユニット構造体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // タプル構造体
struct ChangeColorMessage(i32, i32, i32);

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}


impl Message {
    fn call(&self) {
        // method body would be defined here
        // メソッド本体はここに定義される
        println!("souzou ");
    }
}

impl Animal for Message {}

struct TesterStruct {
    pub state: f64,
}


impl TesterStruct {
    fn test(&self) {
        println!("testest");
    }
}

fn main() {
    let mut status = Status{hash:"test" ,state: 1.0};
    let mut test_rc = Rc::new(TesterStruct{state: 2.12});
}
