use std::collections::VecDeque;
use crate::agent::Agent;

pub struct State;
pub struct Reward(f32);

pub struct MCSTAgent {
    pub memory: VecDeque<(State, State, Reward, bool)>
}
