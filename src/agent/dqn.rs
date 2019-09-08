use std::collections::{VecDeque, HashMap};
use rand_xoshiro::Xoshiro512StarStar;
use rand::{Rng, SeedableRng};
use crate::model::{Point, Action};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct DQNState {
    pub lines_burnt: usize,
    pub sum_holes: u16,
    pub sum_bumps: u16,
    pub sum_height: u16,
    pub curr_shape_idx: usize,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct DQNAction {
    pub base: Point,
    pub rotation: i8,
}

#[derive(Clone, Debug)]
pub struct AgentConf {
    n_neurons: Vec<i32>,       // [32, 32]
    batch_size: i32,           // 512
    activations: Vec<String>,  // ['relu', 'relu', 'linear']
    episodes: i32,             // 2000
    epsilon: f32,              // 1.0
    epsilon_stop_episode: i32, // 2000
    mem_size: i32,             // 25000
    discount: f32,             // 0.99
    replay_start_size: i32,    // 2000
    epochs: i32,               // 1
    render_every: Option<i32>, // None
    train_every: i32,          // 1
    log_every: i32,            // 10
    max_step: Option<i32>,     // Some(10000)
}

impl Default for AgentConf {
    fn default() -> Self {
        AgentConf {
            n_neurons: vec![32, 32],
            batch_size: 512,
            activations: vec!["relu".into(), "relu".into(), "linear".into()],
            episodes: 2000,
            epsilon: 1.0,
            epsilon_stop_episode: 2000,
            mem_size: 25000,
            discount: 0.99,
            replay_start_size: 2000,
            epochs: 1,
            render_every: None,
            train_every: 1,
            log_every: 10,
            max_step: Some(10000),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DQNAgent {
    pub conf: AgentConf,
    pub memory: VecDeque<(DQNState, DQNState, f32, bool)>,
    pub rng: Xoshiro512StarStar,
}

#[derive(Debug, Clone)]
pub struct DQNTransition {
    curr_state: DQNState,
    action: DQNAction,
    reward: f32,
    next_state: DQNState,
    done: bool
}

impl DQNAgent {
    pub fn new(seed: Option<u64>) -> DQNAgent {
        let conf = AgentConf::default();
        let rng = if let Some(seed) = seed {
            Xoshiro512StarStar::seed_from_u64(seed)
        } else {
            Xoshiro512StarStar::from_entropy()
        };
        DQNAgent {
            conf,
            memory: VecDeque::default(),
            rng,
        }
    }
    pub fn select_best_action(&mut self, actions: &[DQNAction]) -> DQNAction {
        //def policy_fn(sess: tf.Session, observation: np.ndarray, epsilon: float) -> np.ndarray:
        //    """
        //    Args:
        //        sess: Tensorflow session
        //        observation: [84, 84, 4] grayscale image, the current and 3 previous frames
        //        epsilon: the probability to choose action randomly instead of via neural network
        //    Returns:
        //        probability vector for every action
        //    """
        //    a = np.ones(n_a, dtype=float) * epsilon / n_a
        //    q_values = estimator.predict(sess, np.expand_dims(observation, 0))[0]
        //    best_action = np.argmax(q_values)
        //    a[best_action] += (1.0 - epsilon)
        //    return a
        if self.rng.gen::<f32>() <= self.conf.epsilon {
            actions[self.rng.gen_range(0, actions.len())]
        } else {
            // ask the neural network about the best value
            // q_values = estimator.predict(sess, np.expand_dims(observation, 0))[0]
            // best_action = np.argmax(q_values)        }
            unreachable!();
        }
    }
}