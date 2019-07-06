#![allow(unused)]

#[macro_use]
extern crate lazy_static;

//const TEST: (String, Point) = ("xxx".into(), Point(0, 0));
use tetris::model::{SHAPES, Shape};

fn main() {
    for shape in &*SHAPES {
        println!("Shapes: {:?}", shape);
    }
}
