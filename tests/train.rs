#![feature(type_ascription)]

use tetris::model::{Point, Field, try_position, rotate, GameState};
use tetris::tetrimino::{Tetrimino, build_tetrimino, I, O, L, J, T, S, Z, Style};
use tetris::train::TetrisEnv;

#[test]
fn test_dqn_state() {
    let field = Field {
        cells: vec![
            vec![1, 0, 0, 0],
            vec![0, 1, 0, 0],
            vec![1, 0, 1, 0],
            vec![1, 0, 1, 0],
            vec![0, 1, 0, 0],
        ],
        height: 5,
        width: 4,
    };
    let mut gs = GameState::initial(5, 4, Default::default(), Some(33));
    gs.field = field;
    let env = TetrisEnv { gs, lines_burnt: 0 };
    let block_heights = env.get_block_heights();
    assert_eq!(vec![5, 4, 3, 0], block_heights);
    assert_eq!(5, env.get_sum_holes(&block_heights));
    assert_eq!(5 + 4 + 3 + 0, env.get_sum_height(&block_heights));
    assert_eq!(1 + 1 + 3, env.get_sum_bumps(&block_heights));
}

