#![allow(unused)]
#![feature(type_ascription)]

#[macro_use]
extern crate lazy_static;

//const TEST: (String, Point) = ("xxx".into(), Point(0, 0));
use tetris::model::{SHAPES, Shape, initial_state};
use rand_xoshiro::Xoroshiro64StarStar;
use rand::{SeedableRng, Rng};

//fn main() {
//    //for shape in &*SHAPES {
//    //    println!("Shapes: {:?}", shape);
//    //}
//
//    let gs = initial_state(10, 20, Some(69));
//    println!("{:?}", gs);
//}


fn main() {
    let mut rng = Xoroshiro64StarStar::seed_from_u64(69);
    println!("{}", rng.gen(): f32);
}