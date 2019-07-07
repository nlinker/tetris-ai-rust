#![allow(unused)]
#![feature(type_ascription)]

#[macro_use]
extern crate lazy_static;

//const TEST: (String, Point) = ("xxx".into(), Point(0, 0));
use tetris::model::{SHAPES, Shape, initial_state};

fn main() {
    let mut gs = initial_state(15, 10, Some(68));
    // curr_shape_idx = 3
    println!("{}", gs);
}