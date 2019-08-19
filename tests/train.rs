#![feature(type_ascription)]

use tetris::model::{Point, Field, try_shape, rotate, GameState, Action};
use tetris::train::TetrisEnv;
use tetris::agent::DQNAction;

#[test]
fn test_dqn_state_after_step() {
    // an impossible state, because in the normal game blocks fall down,
    // but this state is enough to make the test
    let field = Field {
        cells: vec![
            vec![1, 0, 0, 0],
            vec![0, 1, 0, 0],
            vec![1, 0, 1, 0],
            vec![0, 0, 0, 0],
            vec![1, 1, 1, 0],
        ],
        height: 5,
        width: 4,
    };
    let mut gs = GameState::initial(5, 4, Default::default(), Some(7));
    gs.field = field;
    gs.curr_shape_idx = 0;
    gs.rotation = 1;
    gs.base = Point(2, 3);
    gs.curr_cells = gs.try_current_shape(&gs.base, gs.rotation).unwrap();
    let mut env = TetrisEnv { gs, lines_burnt: 0 };
    // the bottom line should be burnt
    // let dqn_action = DQNAction { base: gs.base, rotation: gs.rotation }; <- cannot do here
    let dqn_action = DQNAction {
        base: Point(2, 3),
        rotation: 1
    };
    let (dqn_state, reward, done) = env.step(dqn_action);
    // the cells are = [
    //    [0, 0, 0, 0]
    //    [1, 0, 0, 0],
    //    [0, 1, 0, 1],
    //    [1, 0, 1, 1],
    //    [0, 0, 0, 1],
    // ],
    let block_heights = env.get_block_heights();
    assert_eq!(block_heights, vec![4, 3, 2, 3]);

    assert_eq!(env.lines_burnt, 1);
    assert_eq!(dqn_state.lines_burnt, 1);
    assert_eq!(dqn_state.sum_holes, 2 + 2 + 1);
    assert_eq!(dqn_state.sum_height, 4 + 3 + 2 + 3);
    assert_eq!(dqn_state.sum_bumps, 1 + 1 + 1);
}

#[test]
fn test_get_valid_actions() {
    let field = Field {
        cells: vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![3, 0, 0, 0, 0],
            vec![2, 0, 0, 0, 4],
            vec![2, 3, 0, 4, 4],
            vec![0, 1, 1, 1, 1],
            vec![0, 1, 1, 1, 1],
        ],
        height: 7,
        width: 5,
    };
    let mut gs = GameState::initial(7, 5, Default::default(), Some(7));
    gs.field = field;
    assert_eq!(gs.curr_shape_idx, 0);
    let mut env = TetrisEnv { gs, lines_burnt: 0 };
    let valid_actions = env.get_valid_actions();
    assert_eq!(valid_actions, vec![]);
}
