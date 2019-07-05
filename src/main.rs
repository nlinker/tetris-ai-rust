#![allow(unused)]

//const TEST: (String, Point) = ("xxx".into(), Point(0, 0));
use tetris::model::{I, T, build_piece};
use tetris::utils::Trim;

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




fn main() {
    let t = build_piece(I);
    println!("{:?}", t);
}
