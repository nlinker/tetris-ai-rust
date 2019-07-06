#![allow(unused)]

#[macro_use]
extern crate lazy_static;

//const TEST: (String, Point) = ("xxx".into(), Point(0, 0));
use tetris::model::{I, T, build_shape};
use tetris::utils::Trim;
use std::collections::HashMap;

//fn build_piece(p_str: &str) -> Vec<u8> {
//    let mut cells: Vec<u8> = vec![0; 16];
//    // let mut cells: Vec<Vec<u8>> = vec![vec![0; n]; m];
//    let mut i = 0;
//    for line in p_str.trim_indent().split('\n') {
//        let line: &str = line;
//        let chars = line.chars().filter(|c| !c.is_whitespace()).collect::<Vec<_>>();
//        cells[i * 4 + 0] = if chars[0] == '.' { 0 } else { 1 };
//        cells[i * 4 + 1] = if chars[1] == '.' { 0 } else { 1 };
//        cells[i * 4 + 2] = if chars[2] == '.' { 0 } else { 1 };
//        cells[i * 4 + 3] = if chars[3] == '.' { 0 } else { 1 };
//        i += 1;
//    }
//    cells
//}

lazy_static! {
    pub static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "hello");
        m.insert(1, ",");
        m.insert(2, " ");
        m.insert(3, "world");
        m
    };
    pub static ref COUNT: usize = HASHMAP.len();
}

fn main() {
    // We dereference COUNT because it's type is &usize
    println!("The map has {} entries.", *COUNT);

    // Here we don't dereference with * because of Deref coercions
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());
}
