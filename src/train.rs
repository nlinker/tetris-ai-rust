use crate::agent::{DQNAgent, DQNState, DQNAction};
use crate::model::{GameState, rotate, Point};
use crate::tetrimino::TETRIMINOES;
use std::collections::HashMap;

/// `lines_burnt` - how many lines has been burnt since
/// the last state with the new action applied
#[derive(Debug, Clone)]
pub struct TetrisEnv {
    pub gs: GameState,
    pub lines_burnt: u16,
}

impl TetrisEnv {
    pub fn new(seed: Option<u64>) -> TetrisEnv {
        TetrisEnv {
            gs: GameState::initial(22, 10, Default::default(), seed),
            lines_burnt: 0,
        }
    }

    pub fn reset(&mut self) -> DQNState {
        self.gs.reset();
        self.lines_burnt = 0;
        self.convert_to_dqn_state()
    }

    pub fn step(&mut self, action: DQNAction) -> (DQNState, f32, bool) {
        // TODO finish
        (self.convert_to_dqn_state(), 0.0, false)
    }

    pub fn get_next_transitions(&self) -> HashMap<DQNAction, DQNState> {
        // called after the new piece spawn
        let rotations = match self.gs.curr_shape_idx {
            1 => vec![0], // O
            0 | 3 | 4 => vec![0, 1], // I, S, Z
            2 | 5 | 6 => vec![0, 1, 2, 3], // T, J, L
            _ => unreachable!(),
        };
        let mut transitions = HashMap::new();
        for r in rotations {
            let piece = rotate(&TETRIMINOES[self.gs.curr_shape_idx], r);
            let width1 = self.gs.field.width as i32 - 1;
            let (min_j, max_j) = piece.iter().fold((0, width1),
                                                   |acc, p| (acc.0.min(p.1), acc.1.max(p.1)));
            for j in min_j..max_j {

            }
            // get board props
            // lines_burnt, holes_count, total_bumpiness, sum_height
        }
        transitions
    }

    pub fn get_block_heights(&self) -> Vec<u16> {
        let n = self.gs.field.width;
        let m = self.gs.field.height;
        let mut heights = Vec::with_capacity(n);
        for j in 0..n {
            let mut block_height = 0 as u16;
            for i in 0..m {
                if self.gs.field.cells[i][j] > 0 {
                    block_height = (m - i) as u16;
                    break
                }
            }
            heights.push(block_height);
        }
        heights
    }

    pub fn get_sum_holes(&self, block_heights: &[u16]) -> u16 {
        // iterate through columns, the empty cells inside j-th block we call a 'hole'
        let n = self.gs.field.width;
        let m = self.gs.field.height;
        let mut sum_holes = 0;
        for j in 0..n {
            let m1 = m - block_heights[j] as usize;
            for i in m1..m {
                if self.gs.field.cells[i][j] == 0 {
                    sum_holes += 1;
                }
            }
        }
        sum_holes
    }

    pub fn get_sum_height(&self, block_heights: &[u16]) -> u16 {
        block_heights.iter().sum()
    }

    pub fn get_sum_bumps(&self, block_heights: &[u16]) -> u16 {
        // sum of the differences of heights between pair of columns
        let n = self.gs.field.width;
        let m = self.gs.field.height;
        let mut sum_bumps = 0;
        block_heights.windows(2).for_each(|hs| {
            let h1 = hs[0] as i16;
            let h2 = hs[1] as i16;
            let bump = (h1 - h2).abs();
            sum_bumps += bump as u16;
        });
        sum_bumps
    }

    fn convert_to_dqn_state(&self) -> DQNState {
        let block_heights = self.get_block_heights();
        DQNState {
            lines_burnt: self.lines_burnt,
            sum_holes: self.get_sum_holes(&block_heights),
            sum_bumps: self.get_sum_bumps(&block_heights),
            sum_height: self.get_sum_height(&block_heights),
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