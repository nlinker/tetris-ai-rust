use crate::agent::{DQNAgent, DQNState, DQNAction, AgentConf};
use crate::model::{GameState, rotate, Action, Point, is_valid, try_shape};
use crate::tetrimino::TETRIMINOES;
use std::collections::VecDeque;
use rand_xoshiro::Xoshiro512StarStar;
use rand::{SeedableRng, Rng};
use rand::prelude::SliceRandom;

/// `lines_burnt` - how many lines has been burnt since
/// the last state with the new action applied,
/// essentially it is the part of the previous state
#[derive(Debug, Clone)]
pub struct TetrisEnv {
    pub gs: GameState,
    pub lines_burnt: usize,
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

    pub fn step(&mut self, dqn_action: DQNAction) -> (DQNState, f32, bool) {
        // note: dqn_action should be valid,
        // the last action in the sequence must be Action::HardDrop to get
        // the correct `lines_burnt` value
        let gs = &mut self.gs;
        let old_score = gs.score;
        if let Some(cells) = gs.try_current_shape(&dqn_action.base, dqn_action.rotation) {
            gs.base = dqn_action.base;
            gs.rotation = dqn_action.rotation;
            gs.curr_cells = cells;
        }
        let (lines_burnt, _) = gs.step(Action::HardDrop);
        self.lines_burnt = lines_burnt;
        let reward = (gs.score - old_score) as f32;
        (self.convert_to_dqn_state(), reward, self.gs.game_over)
    }

    pub fn get_valid_actions(&self) -> Vec<DQNAction> {
        // called after the new piece spawn
        let rotations = match self.gs.curr_shape_idx {
            1 => vec![0], // O
            0 | 3 | 4 => vec![0, 1], // I, S, Z
            2 | 5 | 6 => vec![0, 1, 2, 3], // T, J, L
            _ => unreachable!(),
        };
        // in the worst case we have 4 rotations with each base, so the memory
        let n = self.gs.field.width;
        let mut valid_actions = Vec::with_capacity(2 * self.gs.field.width);
        let mut j_shifts = Vec::with_capacity(self.gs.field.width);
        for rotation in rotations {
            j_shifts.clear();
            let mut rotated_shape = rotate(&TETRIMINOES[self.gs.curr_shape_idx], rotation);
            let mut gs_base = self.gs.base;
            if rotation > 0 {
                // we do the complex try_wall_kick_current_shape instead of
                // just rotate, because some shapes can be rotated only with the shift down
                if let Some((base, cells)) = self.gs.try_wall_kick_current_shape((rotation - 1, rotation)) {
                    gs_base = base;
                } else {
                    // no more rotations is possible
                    break;
                }
            }
            // initial position
            {
                let base = Point(gs_base.0, gs_base.1);
                if is_valid(&self.gs.field, &base, &rotated_shape) {
                    j_shifts.push(0 as i32);
                }
            }
            // left
            for dj in 1..n {
                let base = Point(gs_base.0, gs_base.1 - (dj as i32));
                if is_valid(&self.gs.field, &base, &rotated_shape) {
                    j_shifts.push(-(dj as i32));
                } else {
                    break;
                }
            }
            // right
            for dj in 1..n {
                let base = Point(gs_base.0, gs_base.1 + (dj as i32));
                if is_valid(&self.gs.field, &base, &rotated_shape) {
                    j_shifts.push(dj as i32);
                } else {
                    break;
                }
            }
            // populate the actions
            j_shifts.sort();
            for dj in &j_shifts {
                let base = Point(gs_base.0, gs_base.1 + *dj);
                valid_actions.push(DQNAction { base, rotation });
            }
        }
        valid_actions
    }

    pub fn convert_to_dqn_action(actions: Vec<Action>) -> DQNAction {
        unimplemented!()
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
    let rng = if let Some(seed) = seed {
        Xoshiro512StarStar::seed_from_u64(seed)
    } else {
        Xoshiro512StarStar::from_entropy()
    };
    let conf: AgentConf = Default::default();
    let memory: VecDeque<(DQNState, DQNState, f32, bool)> = Default::default();
    let mut agent = DQNAgent { conf, memory, rng };
    let mut env = TetrisEnv::new(seed);
    let episodes = 2000;
    let max_steps = Some(10000);
    let replay_memory_init_size = 50000;

    println!("Populating replay memory...");
    //    state = env.reset()
    //    state = state_processor.process(sess, state)
    //    state = np.stack([state] * 4, axis=2)
    //    for i in range(replay_memory_init_size):
    //        action_probs = policy(sess, state, epsilons[min(total_t, epsilon_decay_steps - 1)])
    //        action = np.random.choice(np.arange(len(action_probs)), p=action_probs)
    //        next_state, reward, done, _ = env.step(VALID_ACTIONS[action])
    //        next_state = state_processor.process(sess, next_state)
    //        next_state = np.append(state[:, :, 1:], np.expand_dims(next_state, 2), axis=2)
    //        replay_memory.append(Transition(state, action, reward, next_state, done))
    //        if done:
    //            state = env.reset()
    //            state = state_processor.process(sess, state)
    //            state = np.stack([state] * 4, axis=2)
    //        else:
    //            state = next_state
    //
    //
    //
    //
    //
    //
    //
    //
    // agent.add_to_memory(current_state, next_states[best_action], reward, done)
    let state = env.reset();
    for _ in 0..replay_memory_init_size {
        let mut steps = 0;
        let state = env.reset();
        while max_steps.is_none() || steps < max_steps.unwrap() {
            let valid_actions = env.get_valid_actions();
            let best_action = agent.best_action(&valid_actions);
            let (next_state, reward, done) = env.step(best_action);
            // next_state, reward, done, _ = env.step(VALID_ACTIONS[action])

            if done {
                break;
            }
        }
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