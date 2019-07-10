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
use termios::{Termios, tcsetattr};
use termios::os::linux::{ICANON, ECHO, TCSANOW};

fn main() {
    let mut gs = GameState::initial(15, 10, Some(69));

    // How can I read one character from stdin without having to hit enter?
    // https://stackoverflow.com/a/37416107/5066426
    let stdin = 0;
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone();
    new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
    let mut buffer = [0;1];
    let mut reader = io::stdin();

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
            // consume stdin, read until its finished,
            // this is done to avoid echoing all key presses after the game loop finished
            // TODO consume stdin
            // if let Ok(_) = reader.read_exact(&mut buffer) { .. }
            if gs.step(Action::Tick) { break; }
            println!("{}", gs.prettify_game_state(true, true));
            k = 0;
        } else {
            k += 1;
        }
        thread::sleep(Duration::from_millis(60));
    }
    println!("{}", gs.prettify_game_state(false, true));
    tcsetattr(stdin, TCSANOW, & termios).unwrap();  // reset the stdin to
}
