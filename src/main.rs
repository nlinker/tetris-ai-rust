#![allow(unused)]
#![feature(type_ascription)]

#[macro_use]
extern crate lazy_static;

//const TEST: (String, Point) = ("xxx".into(), Point(0, 0));
use std::{thread, io};
use std::time::Duration;
use rand_xoshiro::Xoroshiro128StarStar;
use rand::{SeedableRng, Rng, RngCore};

use tetris::model::{Tetrimino, GameState, Action};
use tetris::tetrimino::TETRIMINOES;
use termios::{Termios, tcsetattr};
use termios::os::linux::{ICANON, ECHO, TCSANOW};
use std::io::{Write, Read};

fn main() {
    let stdin = 0; //libc::STDIN_FILENO;
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone();
    new_termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
    let stdout = io::stdout();
    let mut reader = io::stdin();
    let mut buffer = [0;3];  // read exactly 3 bytes
    print!("Hit a key! ");
    stdout.lock().flush().unwrap();
    reader.read_exact(&mut buffer).unwrap();
    println!("You have hit: {:?}", buffer);
    tcsetattr(stdin, TCSANOW, & termios).unwrap();
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