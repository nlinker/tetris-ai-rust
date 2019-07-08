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
    for k in 0..100 {
        gs.step(Action::Tick);
        match k % 4 {
            0 => gs.step(Action::Left),
            1 => gs.step(Action::Right),
            2 => gs.step(Action::RotateCCW),
            3 => gs.step(Action::RotateCW),
            _ => unreachable!(),
        }
        println!("{}", gs.prettify_game_state(true, true));
        thread::sleep(Duration::from_millis(40));
    }
}