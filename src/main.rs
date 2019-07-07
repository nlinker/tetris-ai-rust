#![allow(unused)]
#![feature(type_ascription)]

#[macro_use]
extern crate lazy_static;

//const TEST: (String, Point) = ("xxx".into(), Point(0, 0));
use tetris::model::{SHAPES, Shape, GameState, Action};

fn main() {
    let mut gs = GameState::initial(15, 10, Some(68));
    gs.step(Action::Tick);
    gs.step(Action::Down);
    // curr_shape_idx = 3
    println!("{}", gs.prettify_game_state(true, true));
}