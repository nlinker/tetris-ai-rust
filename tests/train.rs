#![feature(type_ascription)]

use tetris::model::{Point, Field, try_position, rotate, GameState};
use tetris::tetrimino::{TETRIMINOES, Tetrimino, build_tetrimino, I, O, L, J, T, S, Z, Style};
use tetris::train::TetrisEnv;
use tetris::agent::DQNAction;

#[test]
fn test_dqn_state() {
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
    let mut gs = GameState::initial(5, 4, Default::default(), Some(33));
    gs.field = field;
    gs.curr_shape_idx = 0;
    gs.rotation = 1;
    gs.base = Point(2, 3);
    gs.curr_cells = gs.try_current_shape(&gs.base, gs.rotation).unwrap();
    let mut env = TetrisEnv { gs, lines_burnt: 0 };
    // the bottom line should be burnt
    let (dqn_state, reward, done) = env.step(DQNAction { shift: 0, rotation: 1 });
    // assert_eq!(1, env.lines_burnt);
    let block_heights = env.get_block_heights();
    assert_eq!(vec![5, 4, 3, 0], block_heights);
    assert_eq!(5, env.get_sum_holes(&block_heights));
    assert_eq!(5 + 4 + 3 + 0, env.get_sum_height(&block_heights));
    assert_eq!(1 + 1 + 3, env.get_sum_bumps(&block_heights));
}

