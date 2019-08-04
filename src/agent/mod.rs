//! Agents interface and implementations
//!
//! # Example
//!
//! ```no_run
//! fn say_hello() {
//!
//! }
//! ```

pub mod core;
pub mod dqn;
pub mod mcts_qn;

pub use self::core::*;
pub use dqn::*;
pub use mcts_qn::*;
