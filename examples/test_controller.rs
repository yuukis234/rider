use std::fmt::Debug;
use std::rc::Rc;

use rider::libs::devices::controller::*;


fn main() {
    let mut controller_vec = vec!(Controller::KeyboardInput);
    controller_vec.push(Controller::WindowEvent);
    println!("{:?}", controller_vec);
    controller_vec.drain(..1);
    println!("{:?}", controller_vec);
}