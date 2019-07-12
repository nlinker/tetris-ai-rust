#![feature(type_ascription)]

use console::Style;
use tetris::model::{Point, Tetrimino, Field, try_position, rotate, GameState};
use tetris::tetrimino::{build_tetrimino, I, O, L, J, T, S, Z};

#[test]
fn test_conversion() {
    let expected = Tetrimino {
        diffs: vec![Point(0, -3), Point(0, -1), Point(0, 1), Point(0, 3)],
        shift: Point(0, 1),
        style: Style::from_dotted_str("cyan.bold"),
    };
    assert_eq!(build_tetrimino(I), expected);

    let expected = Tetrimino {
        diffs: vec![Point(-1, -1), Point(-1, 1), Point(1, -1), Point(1, 1)],
        shift: Point(1, 1),
        style: Style::from_dotted_str("yellow.bold"),
    };
    assert_eq!(build_tetrimino(O), expected);

    let expected = Tetrimino {
        diffs: vec![Point(-2, 0), Point(0, -2), Point(0, 0), Point(0, 2)],
        shift: Point(0, 0),
        style: Style::from_dotted_str("magenta.bold"),
    };
    assert_eq!(build_tetrimino(T), expected);

    let expected = Tetrimino {
        diffs: vec![Point(-2, 0), Point(-2, 2), Point(0, -2), Point(0, 0)],
        shift: Point(0, 0),
        style: Style::from_dotted_str("green.bold"),
    };
    assert_eq!(build_tetrimino(S), expected);

    let expected = Tetrimino {
        diffs: vec![Point(-2, -2), Point(-2, 0), Point(0, 0), Point(0, 2)],
        shift: Point(0, 0),
        style: Style::from_dotted_str("red.bold"),
    };
    assert_eq!(build_tetrimino(Z), expected);

    let expected = Tetrimino {
        diffs: vec![Point(-2, -2), Point(0, -2), Point(0, 0), Point(0, 2)],
        shift: Point(0, 0),
        style: Style::from_dotted_str("blue.bold"),
    };
    assert_eq!(build_tetrimino(J), expected);

    let expected = Tetrimino {
        diffs: vec![Point(-2, 2), Point(0, -2), Point(0, 0), Point(0, 2)],
        shift: Point(0, 0),
        style: Style::from_dotted_str("white.bold"),
    };
    assert_eq!(build_tetrimino(L), expected);
}

#[test]
fn test_rotate() {
    let piece_t = build_tetrimino(T);

    let expected = vec![Point(-1, 0), Point(0, -1), Point(0, 0), Point(0, 1)];
    assert_eq!(expected, rotate(&piece_t, 0));

    let expected = vec![Point(0, 1), Point(-1, 0), Point(0, 0), Point(1, 0)];
    assert_eq!(expected, rotate(&piece_t, 1));
    assert_eq!(expected, rotate(&piece_t, -3));

    let expected = vec![Point(1, 0), Point(0, 1), Point(0, 0), Point(0, -1)];
    assert_eq!(expected, rotate(&piece_t, 2));
    assert_eq!(expected, rotate(&piece_t, -2));

    let expected = vec![Point(0, -1), Point(1, 0), Point(0, 0), Point(-1, 0)];
    assert_eq!(expected, rotate(&piece_t, 3));
    assert_eq!(expected, rotate(&piece_t, -1));
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
    assert_eq!(expected, try_position(&field, &Point(1, 2), &piece_i, 0));

    // note the same as above, but just the order is different
    let expected = Some(vec![Point(1, 3), Point(1, 2), Point(1, 1), Point(1, 0)]);
    assert_eq!(expected, try_position(&field, &Point(1, 2), &piece_i, 2));

    let expected = Some(vec![Point(0, 2), Point(1, 2), Point(2, 2), Point(3, 2)]);
    assert_eq!(expected, try_position(&field, &Point(2, 2), &piece_i, 1));

    let expected = Some(vec![Point(3, 2), Point(2, 2), Point(1, 2), Point(0, 2)]);
    assert_eq!(expected, try_position(&field, &Point(2, 2), &piece_i, 3));

    let expected = None;
    assert_eq!(expected, try_position(&field, &Point(3, 2), &piece_i, 1));
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
    let mut gs = GameState::initial(6, 4, Some(33));
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
    assert_eq!(expected, gs.field.cells);
}


pub fn build_field(_: &str) -> Field {
    // TODO implement this to be similar to the build_shape
    Field {
        cells: vec![],
        height: 0,
        width: 0,
    }
}
