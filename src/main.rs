#![allow(unused)]
#![feature(type_ascription)]

#[macro_use]
extern crate lazy_static;

//const TEST: (String, Point) = ("xxx".into(), Point(0, 0));
use tetris::model::{SHAPES, Shape, initial_state};

//fn main() {
//    //for shape in &*SHAPES {
//    //    println!("Shapes: {:?}", shape);
//    //}
//
//    let gs = initial_state(10, 20, Some(69));
//    println!("{:?}", gs);
//}


fn main() {
    let mut gs = initial_state(20, 10, Some(68));
    // curr_shape_idx = 3
    println!("{}", gs);
}