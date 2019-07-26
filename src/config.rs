use core::default::Default;

/// - `BurnOnly`: 1 for each line burnt, no matter hom much a time
/// - `PieceAndBurn`:  1 + (lines_burnt ^ 2) * field_width
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Scoring {
    BurnOnly,
    PieceAndBurn,
}

/// This defines how to choose new tetrimino to spawn.
/// - `JustRandom`: next_shape_idx = rng.gen_range(0, TETRIMINOES.len());
/// - `ShuffledQueue`: next_shape_idx = random_deque.pop_back()
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Randomness {
    JustRandom,
    ShuffledQueue,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Config {
    pub scoring: Scoring,
    pub randomness: Randomness,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            scoring: Scoring::BurnOnly,
            randomness: Randomness::JustRandom,
        }
    }
}