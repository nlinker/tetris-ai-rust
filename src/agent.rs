use std::collections::VecDeque;

pub trait Agent {}

pub struct State;
pub struct Reward(f32);

pub struct DQNAgent {
    pub memory: VecDeque<(State, State, Reward, bool)>
}

impl Agent for DQNAgent {

}