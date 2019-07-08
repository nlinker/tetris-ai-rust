#![allow(unused)]
#![feature(type_ascription)]

#[macro_use]
extern crate lazy_static;

//const TEST: (String, Point) = ("xxx".into(), Point(0, 0));
use std::thread;
use std::time::Duration;
use rand_xoshiro::Xoroshiro128StarStar;
use rand::{SeedableRng, Rng, RngCore};

use tetris::model::{Shape, GameState, Action};
use tetris::shapes::SHAPES;

fn main() {
    let mut rng = Xoroshiro128StarStar::from_entropy();

    let mut gs = GameState::initial(15, 10, Some(68));
    for _ in 0..100 {
        gs.step(Action::Tick);
        println!("{}", gs.prettify_game_state(true, true));
        thread::sleep(Duration::from_millis(200));
    }
}