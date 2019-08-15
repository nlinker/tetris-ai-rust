use std::collections::{VecDeque, HashMap};
use crate::model::Point;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct DQNState {
    pub lines_burnt: usize,
    pub sum_holes: u16,
    pub sum_bumps: u16,
    pub sum_height: u16,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct DQNAction {
    pub base: Point,
    pub rotation: i8,
}

#[derive(Default, Debug)]
pub struct AgentConf;

#[derive(Default)]
pub struct DQNAgent {
    pub conf: AgentConf,
    pub memory: VecDeque<(DQNState, DQNState, f32, bool)>
}

impl DQNAgent {
    pub fn select(&self, transitions: HashMap<DQNAction, DQNState>) -> DQNAction {
        unimplemented!("We need to choose the best transition and then the key of the transition")
    }
}