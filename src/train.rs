use crate::agent::{DQNAgent, DQNState, DQNAction};
use crate::model::{GameState, rotate, Point};
use crate::tetrimino::TETRIMINOES;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct TetrisEnv(GameState);

impl TetrisEnv {
    fn new(seed: Option<u64>) -> TetrisEnv {
        TetrisEnv(GameState::initial(22, 10, Default::default(), seed))
    }
    fn reset(&mut self) {
        self.0.reset();
    }
    fn get_next_transitions(&self) -> HashMap<DQNAction, DQNState> {
        // called after the new piece spawn
        let gs = &self.0;
        let rotations = match gs.curr_shape_idx {
            1         => vec![0], // O
            0 | 3 | 4 => vec![0, 1], // I, S, Z
            2 | 5 | 6 => vec![0, 1, 2, 3], // T, J, L
            _         => unreachable!(),
        };
        let mut transitions = HashMap::new();
        for r in rotations {
            let piece = rotate(&TETRIMINOES[gs.curr_shape_idx], r);
            let width1 = gs.field.width as i32 - 1;
            let min_max = piece.iter().fold((0, width1), |acc, p| (acc.0.min(p.1), acc.1.max(p.1)));
        }
        transitions
    }
}

//fn convert_to_dqn_state() -> DQNState {
//
//}

pub fn run_training(seed: Option<u64>) -> failure::Fallible<()> {
    let agent = DQNAgent {
        conf: Default::default(),
        memory: Default::default()
    };
    let mut env = TetrisEnv::new(seed);
    env.reset();
    println!("env = {:?}", env);
    env.reset();
    println!("env = {:?}", env);

    let episodes = 2000;
    let max_steps = Some(10000);
//    for episode in 0..episodes {
//        let mut steps = 0;
//        env.reset();
//        while max_steps.is_none() || steps < max_steps.unwrap() {
//            let next_states = env.get_next_transitions();
//            let best_action = agent.select(next_states);
//            let (reward, done) = env.run(best_action);
//            if done {
//                break
//            }
//        }
//    }
    Ok(())
}