#![allow(unused)]
#![feature(type_ascription)]

#[macro_use]
extern crate lazy_static;

//const TEST: (String, Point) = ("xxx".into(), Point(0, 0));
use std::{thread, io};
use std::time::Duration;
use rand::{SeedableRng, Rng, RngCore};
use rand_xoshiro::Xoroshiro128StarStar;
use std::io::{Write, Read};
use device_query::{DeviceQuery, DeviceState, MouseState, Keycode};

use tetris::model::{Tetrimino, GameState, Action};
use tetris::tetrimino::TETRIMINOES;

fn main() {
    let mut gs = GameState::initial(20, 16, Some(68));

    let device_state = DeviceState::new();
    let mut k = 0;
    let mut key_press: Option<Keycode> = None;
    loop {
        let keys: Vec<Keycode> = device_state.query_keymap();
        if !keys.is_empty() {
            let x = match keys[0] {
                Keycode::A => { gs.step(Action::Left); true }
                Keycode::D => { gs.step(Action::Right); true }
                Keycode::S => { gs.step(Action::Down); true }
                _ => false,
            };
            if x {
                println!("{}", gs.prettify_game_state(true, true));
            }
            if key_press.is_none() {
                // special case for rotation to disable echoing
                key_press = Some(keys[0].clone());
                match keys[0] {
                    Keycode::W => {
                        gs.step(Action::RotateCW);
                        println!("{}", gs.prettify_game_state(true, true));
                    },
                    Keycode::Q => {
                        gs.step(Action::RotateCCW);
                        println!("{}", gs.prettify_game_state(true, true));
                    },
                    _ => {}
                }
            }
        } else {
            // reset the state, when key is released
            key_press = None;
        }
        if k >= 10 {
            if gs.step(Action::Tick) { break; }
            println!("{}", gs.prettify_game_state(true, true));
            k = 0;
        } else {
            k += 1;
        }
        thread::sleep(Duration::from_millis(60));
    }
    println!("{}", gs.prettify_game_state(false, true));

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