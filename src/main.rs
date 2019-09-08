// #![allow(unused)]
#![feature(type_ascription)]

use clap::{App, Arg};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::async_stdin;
use std::io::{Write, stdout};
use std::{thread, io};
use std::time::Duration;
use core::default::Default;
use tch::{nn, nn::ModuleT, nn::OptimizerConfig, Device, Tensor, Cuda};
use tetris::model::{GameState, Action};
use tetris::agent::{DQNAgent, DQNState};
use tetris::train::run_training;
use tetris::config::{Config, Scoring, Randomness};
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "tetris-app", about = "Runs various command against tetris environment")]
struct Opt {
    /// The mode how to run the program
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short = "m", long = "mode")]
    mode: Mode
}

#[derive(Debug)]
enum Mode {
    Run, Train, Mnist
}

impl FromStr for Mode {
    type Err = String;
    fn from_str(day: &str) -> Result<Self, Self::Err> {
        match day {
            "run" => Ok(Mode::Run),
            "train" => Ok(Mode::Train),
            "mnist" => Ok(Mode::Mnist),
            _ => Err("Could not parse a day".into()),
        }
    }
}

// io::Result<()>
fn main() -> failure::Fallible<()> {
    let opt: Opt = Opt::from_args();
    match opt.mode {
        Mode::Run => run_interactive_game(),
        Mode::Train => run_training(None),
        Mode::Mnist => run_training_mnist(),
    }
}

fn run_interactive_game() -> failure::Fallible<()> {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = async_stdin().keys();

    write!(stdout, "{}{}{}", termion::clear::All, termion::cursor::Goto(1, 2), termion::cursor::Hide).unwrap();
    stdout.flush().unwrap();

    let config = Config {
        scoring: Scoring::BurnOnly,
        randomness: Randomness::ShuffledQueue,
    };
    let mut gs = GameState::initial(22, 10, config, Some(22));
    let mut k = 0;
    let k_delay = 80;
    {
        println!("{}", gs.prettify_game_state(true, true, true));
        stdout.flush().unwrap();
    }
    loop {
        if let Some(c) = stdin.next() {
            let key = c.unwrap().clone();
            let x = match &key {
                Key::Ctrl('c') => { break; },
                Key::Char(' ') => { gs.step(Action::HardDrop); k = 0; true },
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
        if k >= k_delay {
            let _ = gs.step(Action::Tick);
            println!("{}", gs.prettify_game_state(true, true, true));
            stdout.flush().unwrap();
            k = 0;
        } else {
            k += 1;
        }
        if gs.game_over { break; }
        thread::sleep(Duration::from_millis(10));
    }
    write!(stdout, "{}", gs.prettify_game_state(false, true, true)).unwrap();
    write!(stdout, "{}", termion::cursor::Show).unwrap();
    stdout.flush().unwrap();
    Ok(())
}

fn run_training_mnist() -> failure::Fallible<()> {
    #[derive(Debug)]
    struct Net {
        conv1: nn::Conv2D,
        conv2: nn::Conv2D,
        fc1: nn::Linear,
        fc2: nn::Linear,
    }

    impl Net {
        fn new(vs: &nn::Path) -> Net {
            let conv1 = nn::conv2d(vs, 1, 32, 5, Default::default());
            let conv2 = nn::conv2d(vs, 32, 64, 5, Default::default());
            let fc1 = nn::linear(vs, 1024, 1024, Default::default());
            let fc2 = nn::linear(vs, 1024, 10, Default::default());
            Net {
                conv1,
                conv2,
                fc1,
                fc2,
            }
        }
    }

    impl nn::ModuleT for Net {
        fn forward_t(&self, xs: &Tensor, train: bool) -> Tensor {
            xs.view([-1, 1, 28, 28])
                .apply(&self.conv1)
                .max_pool2d_default(2)
                .apply(&self.conv2)
                .max_pool2d_default(2)
                .view([-1, 1024])
                .apply(&self.fc1)
                .relu()
                .dropout_(0.5, train)
                .apply(&self.fc2)
        }
    }
    let m = tch::vision::mnist::load_dir("data")?;
    println!("Cuda::is_available() = {}", Cuda::is_available());
    println!("Cuda::cudnn_is_available() = {}", Cuda::cudnn_is_available());
    let vs = nn::VarStore::new(Device::cuda_if_available());
    let net = Net::new(&vs.root());
    let opt = nn::Adam::default().build(&vs, 1e-4)?;
    for epoch in 1..100 {
        for (bimages, blabels) in m.train_iter(256).shuffle().to_device(vs.device()) {
            let loss = net
                .forward_t(&bimages, true)
                .cross_entropy_for_logits(&blabels);
            opt.backward_step(&loss);
        }
        let test_accuracy =
            net.batch_accuracy_for_logits(&m.test_images, &m.test_labels, vs.device(), 1024);
        println!("epoch: {:4} test acc: {:5.2}%", epoch, 100. * test_accuracy,);
    }
    Ok(())
}
