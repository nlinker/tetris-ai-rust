#![feature(type_ascription)]

//use std::collections::HashSet;
use tetris::model::{Point, convert_piece, I, O, L, J, T, S, Z, Piece, Field, try_position, rotate};

#[test]
fn test_conversion() {
    let expected = Piece {
        diffs: vec![Point(0, -3), Point(0, -1), Point(0, 1), Point(0, 3)],
        shift: Point(0, 1),
    };
    assert_eq!(expected, convert_piece(I));

    let expected = Piece {
        diffs: vec![Point(-1, -1), Point(-1, 1), Point(1, -1), Point(1, 1)],
        shift: Point(1, 1),
    };
    assert_eq!(expected, convert_piece(O));

    let expected = Piece {
        diffs: vec![Point(-2, -1), Point(0, -1), Point(2, -1), Point(2, 1)],
        shift: Point(0, 1),
    };
    assert_eq!(expected, convert_piece(L));

    let expected = Piece {
        diffs: vec![Point(-2, 1), Point(0, 1), Point(2, -1), Point(2, 1)],
        shift: Point(0, 1),
    };
    assert_eq!(expected, convert_piece(J));

    let expected = Piece {
        diffs: vec![Point(-1, -2), Point(-1, 0), Point(-1, 2), Point(1, 0)],
        shift: Point(1, 0),
    };
    assert_eq!(expected, convert_piece(T));

    let expected = Piece {
        diffs: vec![Point(-1, 0), Point(-1, 2), Point(1, -2), Point(1, 0)],
        shift: Point(1, 0),
    };
    assert_eq!(expected, convert_piece(S));

    let expected = Piece {
        diffs: vec![Point(-1, -2), Point(-1, 0), Point(1, 0), Point(1, 2)],
        shift: Point(1, 0),
    };
    assert_eq!(expected, convert_piece(Z));
}

#[test]
fn test_rotate() {
    let piece_t = convert_piece(T);

    let expected = vec![Point(-1, -1), Point(-1, 0), Point(-1, 1), Point(0, 0)];
    assert_eq!(expected, rotate(&piece_t, 0));

    let expected = vec![Point(1, -1), Point(0, -1), Point(-1, -1), Point(0, 0)];
    assert_eq!(expected, rotate(&piece_t, 1));

    let expected = vec![Point(0, 1), Point(0, 0), Point(0, -1), Point(-1, 0)];
    assert_eq!(expected, rotate(&piece_t, 2));

    let expected = vec![Point(-1, 0), Point(0, 0), Point(1, 0), Point(0, -1)];
    assert_eq!(expected, rotate(&piece_t, 3));
}

fn test_try_position() {
    let field = Field {
        cells: vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ],
        height: 4,
        width: 4,
    };
    let piece_i = convert_piece(I);
    assert_eq!(None, try_position(&field, &Point(1, 1), &piece_i, 0));
}

pub fn convert_field(_: &str) -> Field {
    // TODO implement this to
    Field {
        cells: vec![],
        height: 0,
        width: 0,
    }
}
