#![feature(type_ascription)]

use tetris::model::{Point, Field, try_shape, rotate, GameState, Action};
use tetris::tetrimino::{Tetrimino, build_tetrimino, I, O, L, J, T, S, Z, Style};

#[test]
fn test_build_tetrimino_i() {
    let expected = Tetrimino {
        diffs: vec![Point(-1, -3), Point(-1, -1), Point(-1, 1), Point(-1, 3)],
        parity: Point(1, 1),
        style: Style::Cyan,
    };
    assert_eq!(build_tetrimino(I), expected);
}

#[test]
fn test_build_tetrimino_o() {
    let expected = Tetrimino {
        diffs: vec![Point(-1, -1), Point(-1, 1), Point(1, -1), Point(1, 1)],
        parity: Point(1, 1),
        style: Style::Yellow,
    };
    assert_eq!(build_tetrimino(O), expected);
}

#[test]
fn test_build_tetrimino_t() {
    let expected = Tetrimino {
        diffs: vec![Point(-2, 0), Point(0, -2), Point(0, 0), Point(0, 2)],
        parity: Point(0, 0),
        style: Style::Magenta,
    };
    assert_eq!(build_tetrimino(T), expected);
}

#[test]
fn test_build_tetrimino_s() {
    let expected = Tetrimino {
        diffs: vec![Point(-2, 0), Point(-2, 2), Point(0, -2), Point(0, 0)],
        parity: Point(0, 0),
        style: Style::Green,
    };
    assert_eq!(build_tetrimino(S), expected);
}

#[test]
fn test_build_tetrimino_z() {
    let expected = Tetrimino {
        diffs: vec![Point(-2, -2), Point(-2, 0), Point(0, 0), Point(0, 2)],
        parity: Point(0, 0),
        style: Style::Red,
    };
    assert_eq!(build_tetrimino(Z), expected);
}

#[test]
fn test_build_tetrimino_j() {
    let expected = Tetrimino {
        diffs: vec![Point(-2, -2), Point(0, -2), Point(0, 0), Point(0, 2)],
        parity: Point(0, 0),
        style: Style::Blue,
    };
    assert_eq!(build_tetrimino(J), expected);
}

#[test]
fn test_build_tetrimino_l() {
    let expected = Tetrimino {
        diffs: vec![Point(-2, 2), Point(0, -2), Point(0, 0), Point(0, 2)],
        parity: Point(0, 0),
        style: Style::White,
    };
    assert_eq!(build_tetrimino(L), expected);
}

#[test]
fn test_rotate_i() {
    let piece_t = build_tetrimino(I);

    let expected = vec![
        vec![Point(0, -1), Point(0, 0), Point(0, 1), Point(0, 2)],
        vec![Point(-1, 1), Point(0, 1), Point(1, 1), Point(2, 1)],
        vec![Point(1, 2), Point(1, 1), Point(1, 0), Point(1, -1)],
        vec![Point(2, 0), Point(1, 0), Point(0, 0), Point(-1, 0)],
    ];
    assert_eq!(rotate(&piece_t, 0), expected[0]);
    assert_eq!(rotate(&piece_t, 1), expected[1]);
    assert_eq!(rotate(&piece_t, 2), expected[2]);
    assert_eq!(rotate(&piece_t, 3), expected[3]);
}

#[test]
fn test_rotate_t() {
    let piece_t = build_tetrimino(T);

    let expected = vec![
        vec![Point(-1, 0), Point(0, -1), Point(0, 0), Point(0, 1)],
        vec![Point(0, 1), Point(-1, 0), Point(0, 0), Point(1, 0)],
        vec![Point(1, 0), Point(0, 1), Point(0, 0), Point(0, -1)],
        vec![Point(0, -1), Point(1, 0), Point(0, 0), Point(-1, 0)],
    ];
    assert_eq!(rotate(&piece_t, 0), expected[0]);
    assert_eq!(rotate(&piece_t, 1), expected[1]);
    assert_eq!(rotate(&piece_t, 2), expected[2]);
    assert_eq!(rotate(&piece_t, 3), expected[3]);
}

#[test]
fn test_try_position() {
    let field = Field {
        cells: vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 1, 1, 1],
        ],
        height: 5,
        width: 4,
    };
    let piece_i = build_tetrimino(I);

    let expected = Some(vec![Point(1, 0), Point(1, 1), Point(1, 2), Point(1, 3)]);
    assert_eq!(try_shape(&field, &Point(1, 1), 0, &piece_i), expected);

    let expected = Some(vec![Point(0, 2), Point(1, 2), Point(2, 2), Point(3, 2)]);
    assert_eq!(try_shape(&field, &Point(1, 1), 1, &piece_i), expected);

    let expected = Some(vec![Point(2, 3), Point(2, 2), Point(2, 1), Point(2, 0)]);
    assert_eq!(try_shape(&field, &Point(1, 1), 2, &piece_i), expected);

    let expected = Some(vec![Point(3, 1), Point(2, 1), Point(1, 1), Point(0, 1)]);
    assert_eq!(try_shape(&field, &Point(1, 1), 3, &piece_i), expected);

    let expected = None;
    assert_eq!(try_shape(&field, &Point(0, 1), 1, &piece_i), expected);
}

#[test]
fn test_burn() {
    let field = Field {
        cells: vec![
            vec![3, 0, 0, 0],
            vec![2, 0, 0, 0],
            vec![2, 3, 4, 4],
            vec![2, 3, 0, 4],
            vec![1, 1, 3, 4],
            vec![0, 1, 1, 1],
        ],
        height: 6,
        width: 4,
    };
    let mut gs = GameState::initial(6, 4, Default::default(), Some(33));
    gs.field = field;
    gs.burn_lines();
    let expected: Vec<Vec<u8>> = vec![
        vec![0, 0, 0, 0],
        vec![0, 0, 0, 0],
        vec![3, 0, 0, 0],
        vec![2, 0, 0, 0],
        vec![2, 3, 0, 4],
        vec![0, 1, 1, 1],
    ];
    assert_eq!(gs.field.cells, expected);
}

#[test]
fn test_hard_drop() {
    // the situation when hard drop makes impossible to spawn the next piece
    let field = Field {
        cells: vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 1],
            vec![1, 1, 0, 1],
        ],
        height: 6,
        width: 4,
    };
    // this seed provides spawning O-shape
    let mut gs = GameState::initial(6, 4, Default::default(), Some(22));
    assert_eq!(gs.curr_shape_idx, 1);
    gs.field = field;
    let (lines_burnt, done) = gs.step(Action::HardDrop);
    assert_eq!(lines_burnt, 0);
    assert_eq!(done, true);
}


pub fn build_field(_: &str) -> Field {
    // TODO implement this to be similar to the build_shape
    Field {
        cells: vec![],
        height: 0,
        width: 0,
    }
}
