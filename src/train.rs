use crate::agent::{DQNAgent, DQNState, DQNAction, DQNReward};
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

    pub fn step(&mut self, action: DQNAction) -> (DQNState, DQNReward, bool) {
        // TODO finish
        (self.convert_to_dqn_state(), DQNReward(0.0), false)
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

    pub fn get_holes(&self) -> Vec<u16> {
        // iterate through columns, the empty square in each column with block above we call holes
        let n = self.gs.field.width;
        let m = self.gs.field.height;
        let mut holes = Vec::with_capacity(n);
        for j in 0..n {
            let mut m1 = m;
            for i in 0..m {
                if self.gs.field.cells[i][j] > 0 {
                    m1 = i;
                    break
                }
            }
            let mut holes_count = 0;
            for i in m1..m {
                if self.gs.field.cells[i][j] == 0 {
                    holes_count += 1;
                }
            }
            holes.push(holes_count);
        }
        holes
    }

//   def _number_of_holes(self, board):
//        """Number of holes in the board (empty square with at least one block above it)"""
//        holes = 0
//
//        for col in zip(*board):
//            tail = itertools.dropwhile(lambda x: x != Tetris.MAP_BLOCK, col)
//            holes += len([x for x in tail if x == Tetris.MAP_EMPTY])
//
//        return holes

    fn convert_to_dqn_state(&self) -> DQNState {
        DQNState {
            lines_burnt: self.lines_burnt,
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