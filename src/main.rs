#![allow(unused)]

//const TEST: (String, Point) = ("xxx".into(), Point(0, 0));
use tetris::model::I;
use tetris::utils::Trim;

fn build_piece(p_str: &str) -> Vec<Vec<String>> {
    let x = p_str.split('\n');
    unreachable!()
}


fn main() {
    println!("{}", I.trim_indent());
}
