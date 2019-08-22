#![feature(type_ascription)]

use tetris::model::{Point, Field, try_shape, rotate, GameState, Action};
use tetris::train::TetrisEnv;
use tetris::agent::DQNAction;
use std::collections::HashMap;

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
    gs.game_over = false;
    gs.curr_shape_idx = 0;
    gs.rotation = 1;
    gs.base = Point(1, 2);
    gs.curr_cells = gs.try_current_shape(&gs.base, gs.rotation).unwrap();
    // the bottom line should be burnt
    // let dqn_action = DQNAction { base: gs.base, rotation: gs.rotation }; <- cannot do here
    let dqn_action = DQNAction {
        base: gs.base,
        rotation: gs.rotation,
    };
    let mut env = TetrisEnv { gs, lines_burnt: 0 };
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
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 4, 0],
            vec![2, 3, 0, 4, 4],
            vec![0, 1, 1, 1, 1],
            vec![0, 1, 1, 1, 1],
        ],
        height: 7,
        width: 5,
    };
    let mut gs = GameState::initial(field.height, field.width, Default::default(), Some(30));
    gs.field = field;
    let mut env = TetrisEnv { gs, lines_burnt: 0 };
    // note, the action DQNAction { base: Point(3, 1), rotation: 1 } doesn't included
    let expected = vec![
        DQNAction { base: Point(1, 1), rotation: 0 },
        DQNAction { base: Point(1, 2), rotation: 0 },
        DQNAction { base: Point(1, -1), rotation: 1 },
        DQNAction { base: Point(1, 0), rotation: 1 },
        DQNAction { base: Point(1, 1), rotation: 1 },
    ];
    assert_eq!(gs.curr_shape_idx, 0);
    assert_eq!(env.get_valid_actions(), expected);
}

#[test]
fn test_select_actions() {
    let field = Field {
        cells: vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 4, 0],
            vec![2, 3, 0, 4, 4],
            vec![0, 1, 1, 1, 1],
            vec![0, 1, 1, 1, 1],
        ],
        height: 7,
        width: 5,
    };
    let mut gs = GameState::initial(field.height, field.width, Default::default(), Some(30));
    gs.field = field;
    let mut env = TetrisEnv { gs, lines_burnt: 0 };

    assert_eq!(gs.curr_shape_idx, 0);
}

#[test]
fn test_game_state_seeds() {
    let mut seeds_map: HashMap<usize, Vec<u64>> = HashMap::new();
    for seed in 0..100 {
        let gs = GameState::initial(6, 4, Default::default(), Some(seed));
        if let Some(mut seeds) = seeds_map.get_mut(&gs.curr_shape_idx) {
            seeds.push(seed);
        } else {
            seeds_map.insert(gs.curr_shape_idx, vec![]);
        }
    }
    let mut expected = HashMap::new();
    expected.insert(0, vec![9, 21, 29, 30, 37, 38, 39, 40, 42, 45, 64, 69, 73, 79, 96]);
    expected.insert(1, vec![11, 13, 22, 26, 28, 50, 57, 74, 78, 82, 89, 93, 94]);
    expected.insert(2, vec![12, 23, 27, 33, 35, 47, 51, 54, 55, 62, 70, 97, 99]);
    expected.insert(3, vec![31, 52, 56, 58, 60, 61, 66, 67, 76, 77, 80, 81, 83, 88, 95]);
    expected.insert(4, vec![3, 7, 15, 17, 18, 32, 36, 46, 49, 59, 63, 68, 84, 85, 86, 91]);
    expected.insert(5, vec![8, 14, 19, 20, 24, 34, 41, 44, 71, 72, 90, 92]);
    expected.insert(6, vec![16, 25, 43, 48, 53, 65, 75, 87, 98]);
    assert_eq!(seeds_map, expected);
}
