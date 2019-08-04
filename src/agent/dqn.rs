use std::collections::VecDeque;
use crate::agent::Agent;

pub struct State;
pub struct Reward(f32);

#[derive(Default)]
pub struct AgentConf;

pub struct DQNAgent {
    pub conf: AgentConf,
    pub memory: VecDeque<(State, State, Reward, bool)>
}

impl Agent for DQNAgent {

}