use crate::agent::{DQNAgent, DQNState, DQNAction, DQNReward};
use crate::model::{GameState, rotate, Point};
use crate::tetrimino::TETRIMINOES;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct TetrisEnv(GameState);

impl TetrisEnv {
    fn new(seed: Option<u64>) -> TetrisEnv {
        TetrisEnv(GameState::initial(22, 10, Default::default(), seed))
    }

    fn reset(&mut self) -> DQNState {
        self.0.reset();
        self.convert_to_state()
    }

    fn step(&mut self, action: DQNAction) -> (DQNState, DQNReward, bool) {
        // let actions = self.derive_actions(actions);
        // next_state, reward, done
        (self.convert_to_state(), DQNReward(0.0), false) // TODO finish
    }

    fn get_next_transitions(&self) -> HashMap<DQNAction, DQNState> {
        // called after the new piece spawn
        let gs = &self.0;
        let rotations = match gs.curr_shape_idx {
            1 => vec![0], // O
            0 | 3 | 4 => vec![0, 1], // I, S, Z
            2 | 5 | 6 => vec![0, 1, 2, 3], // T, J, L
            _ => unreachable!(),
        };
        let mut transitions = HashMap::new();
        for r in rotations {
            let piece = rotate(&TETRIMINOES[gs.curr_shape_idx], r);
            let width1 = gs.field.width as i32 - 1;
            let (min_j, max_j) = piece.iter().fold((0, width1),
                                                   |acc, p| (acc.0.min(p.1), acc.1.max(p.1)));
            for j in min_j..max_j {

            }
            // get board props
            // lines_burnt, holes_count, total_bumpiness, sum_height
        }
        transitions
    }

    fn convert_to_state(&self) -> DQNState {
        DQNState {
            lines_burnt: 0,
            holes_count: 0,
            total_bumpiness: 0,
            sum_height: 0
        }
    }

}


pub fn run_training(seed: Option<u64>) -> failure::Fallible<()> {
    let agent = DQNAgent {
        conf: Default::default(),
        memory: Default::default(),
    };
    let mut env = TetrisEnv::new(seed);
    let episodes = 2000;
    let max_steps = Some(10000);
    let replay_memory_init_size = 50000;

    println!("Populating replay memory...");
    let mut state = env.reset();
    for _ in 0..replay_memory_init_size {

    }

//    for episode in 0..episodes {
//        let mut steps = 0;
//        let state = env.reset();
//        while max_steps.is_none() || steps < max_steps.unwrap() {
//            let next_states = env.get_next_transitions();
//            let best_action = agent.select(next_states);
//            let (next_state, reward, done) = env.step(best_action);
//            // next_state, reward, done, _ = env.step(VALID_ACTIONS[action])
//
//            if done {
//                break;
//            }
//        }
//    }
    Ok(())
}