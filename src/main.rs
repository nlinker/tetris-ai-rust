// #![allow(unused)]
#![feature(type_ascription)]

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::async_stdin;
use std::io::{Write, stdout};
use std::thread;
use std::time::Duration;

use tetris::model::{GameState, Action};

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = async_stdin().keys();

    write!(stdout, "{}{}{}", termion::clear::All, termion::cursor::Goto(1, 2), termion::cursor::Hide).unwrap();
    stdout.flush().unwrap();

    let mut gs = GameState::initial(22, 10, None);
    let mut k = 0;
    loop {
        if let Some(c) = stdin.next() {
            let k = c.unwrap().clone();
            let x = match &k {
                Key::Ctrl('c') => { break; },
                Key::Char(' ') => { gs.step(Action::HardDrop); true },
                Key::Left      => { gs.step(Action::Left); true }
                Key::Right     => { gs.step(Action::Right); true }
                Key::Down      => { gs.step(Action::Down); true }
                Key::Up        => { gs.step(Action::RotateCW); true }
                Key::End       => { gs.step(Action::RotateCCW); true }
                _              => false,
            };
            if x {
                println!("{}", gs.prettify_game_state(true, true, true));
                stdout.flush().unwrap();
            }
        }
        if k >= 80 {
            if gs.step(Action::Tick) { break; }
            println!("{}", gs.prettify_game_state(true, true, true));
            stdout.flush().unwrap();
            k = 0;
        } else {
            k += 1;
        }
        thread::sleep(Duration::from_millis(10));
    }
    write!(stdout, "{}", gs.prettify_game_state(false, true, true)).unwrap();
    write!(stdout, "{}", termion::cursor::Show).unwrap();
    stdout.flush().unwrap();
}