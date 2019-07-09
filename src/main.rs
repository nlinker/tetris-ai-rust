#![allow(unused)]
#![feature(type_ascription)]

#[macro_use]
extern crate lazy_static;

//const TEST: (String, Point) = ("xxx".into(), Point(0, 0));
use std::{thread, io};
use std::time::Duration;
use std::io::{Write, Read};
use rand_xoshiro::Xoroshiro128StarStar;
use rand::{SeedableRng, Rng, RngCore};

use tetris::model::{Tetrimino, GameState, Action};
use tetris::tetrimino::TETRIMINOES;

use ncurses::*; // watch for globs
use termios::*;

fn main() {
    let mut gs = GameState::initial(20, 16, Some(68));

    let window = initscr();
    nodelay(window, true);
    start_color();
    loop {
        // get keyboard input, returns -1 if none available
        let c = wgetch(window);
        if c != -1 {
            // print numeric value
            init_pair(1, COLOR_BLACK, COLOR_RED);
            init_pair(2, COLOR_BLACK, COLOR_GREEN);

            wmove(window, 0, 0);
            attron(COLOR_PAIR(1));
            waddstr(window, "ERROR: I like tacos, but I don't have any.\n");

            attron(COLOR_PAIR(2));
            waddstr(window, "SUCCESS: I found some tacos.\n");
            refresh();

            // waddstr(window, gs.prettify_game_state(true, true).as_str());
            // wrefresh(window);
            // return curser to start position
        }
        thread::sleep(Duration::from_millis(10));
    }

//
//
//    let mut rng = Xoroshiro128StarStar::from_entropy();
//
//    let mut gs = GameState::initial(20, 16, Some(68));
//    for k in 0..1000 {
//        if gs.step(Action::Tick) { break; }
//        match k % 2 {
//            0 => {
//                for _ in 0..rng.gen_range(1, 8) {
//                    gs.step(Action::Left);
//                    println!("{}", gs.prettify_game_state(true, true));
//                }
//            },
//            1 => {
//                for _ in 0..rng.gen_range(1, 8) {
//                    gs.step(Action::Right);
//                    println!("{}", gs.prettify_game_state(true, true));
//                }
//            },
//            _ => unreachable!(),
//        }
//        if k % 10 == 0 {
//            gs.step(Action::RotateCCW);
//        }
//        println!("{}", gs.prettify_game_state(true, true));
//        thread::sleep(Duration::from_millis(40));
//    }
//    println!("{}", gs.prettify_game_state(false, true));
}