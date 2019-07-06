#![allow(unused)]

#[macro_use]
extern crate lazy_static;

//const TEST: (String, Point) = ("xxx".into(), Point(0, 0));
use tetris::model::{SHAPES, Shape, initial_state};
use xorshift128plus::XorShift128Plus;

//fn main() {
//    //for shape in &*SHAPES {
//    //    println!("Shapes: {:?}", shape);
//    //}
//
//    let gs = initial_state(10, 20, Some(69));
//    println!("{:?}", gs);
//}


fn main() {
    let mut random = XorShift128Plus::from_u64(69);
    println!("{}", random.next());
}