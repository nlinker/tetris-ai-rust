#![feature(type_ascription)]

use tetris::model::{Point, Field, try_position, rotate, GameState};
use tetris::tetrimino::{Tetrimino, build_tetrimino, I, O, L, J, T, S, Z, Style};
use tetris::train::TetrisEnv;

#[test]
fn test_holes_count() {
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
    let expected = vec![2, 2, 1, 0];
    assert_eq!(expected, env.get_holes())
}