use std::collections::VecDeque;

pub struct State;
pub struct Reward(f32);

pub struct MCSTAgent {
    pub memory: VecDeque<(State, State, Reward, bool)>
}
