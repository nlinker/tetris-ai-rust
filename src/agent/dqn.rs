use std::collections::{VecDeque, HashMap};

pub struct DQNState {
    pub lines_burnt: u16,
    pub holes_count: u16,
    pub total_bumpiness: u16,
    pub sum_height: u16,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct DQNAction {
    shift: i16,
    rotation: i8,
}

pub struct DQNReward(pub f32);

#[derive(Default)]
pub struct AgentConf;

#[derive(Default)]
pub struct DQNAgent {
    pub conf: AgentConf,
    pub memory: VecDeque<(DQNState, DQNState, DQNReward, bool)>
}

impl DQNAgent {
    pub fn select(&self, transitions: HashMap<DQNAction, DQNState>) -> DQNAction {
        unimplemented!("We need to choose the best transition and then the key of the transition")
    }
}