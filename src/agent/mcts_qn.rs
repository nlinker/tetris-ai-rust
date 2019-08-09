use std::collections::VecDeque;

struct State;
struct Reward(f32);

struct MCSTAgent {
    memory: VecDeque<(State, State, Reward, bool)>
}
