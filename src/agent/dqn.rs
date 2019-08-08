use std::collections::{VecDeque, HashMap};

pub struct DQNState;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct DQNAction {
    shift: i32,
    rotation: i8,
}

pub struct Reward(f32);

#[derive(Default)]
pub struct AgentConf;

#[derive(Default)]
pub struct DQNAgent {
    pub conf: AgentConf,
    pub memory: VecDeque<(DQNState, DQNState, Reward, bool)>
}

impl DQNAgent {
    fn select(&self, transitions: HashMap<DQNAction, DQNState>) -> DQNAction {
        unimplemented!("We need to choose the best transition and then the key of the transition")
    }
}