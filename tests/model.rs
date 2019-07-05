#![feature(type_ascription)]

//use std::collections::HashSet;
use tetris::model::{Point, build_piece, I, O, L, J, T, S, Z, Shape, Field, try_position, rotate};

#[test]
fn test_conversion() {
    let expected = Shape {
        diffs: vec![Point(0, -3), Point(0, -1), Point(0, 1), Point(0, 3)],
        shift: Point(0, 1),
    };
    assert_eq!(expected, build_piece(I));

    let expected = Shape {
        diffs: vec![Point(-1, -1), Point(-1, 1), Point(1, -1), Point(1, 1)],
        shift: Point(1, 1),
    };
    assert_eq!(expected, build_piece(O));

    let expected = Shape {
        diffs: vec![Point(-2, -1), Point(0, -1), Point(2, -1), Point(2, 1)],
        shift: Point(0, 1),
    };
    assert_eq!(expected, build_piece(L));

    let expected = Shape {
        diffs: vec![Point(-2, 1), Point(0, 1), Point(2, -1), Point(2, 1)],
        shift: Point(0, 1),
    };
    assert_eq!(expected, build_piece(J));

    let expected = Shape {
        diffs: vec![Point(-1, -2), Point(-1, 0), Point(-1, 2), Point(1, 0)],
        shift: Point(1, 0),
    };
    assert_eq!(expected, build_piece(T));

    let expected = Shape {
        diffs: vec![Point(-1, 0), Point(-1, 2), Point(1, -2), Point(1, 0)],
        shift: Point(1, 0),
    };
    assert_eq!(expected, build_piece(S));

    let expected = Shape {
        diffs: vec![Point(-1, -2), Point(-1, 0), Point(1, 0), Point(1, 2)],
        shift: Point(1, 0),
    };
    assert_eq!(expected, build_piece(Z));
}

#[test]
fn test_rotate() {
    let piece_t = build_piece(T);

    let expected = vec![Point(-1, -1), Point(-1, 0), Point(-1, 1), Point(0, 0)];
    assert_eq!(expected, rotate(&piece_t, 0));

    let expected = vec![Point(1, -1), Point(0, -1), Point(-1, -1), Point(0, 0)];
    assert_eq!(expected, rotate(&piece_t, 1));
    assert_eq!(expected, rotate(&piece_t, -3));

    let expected = vec![Point(0, 1), Point(0, 0), Point(0, -1), Point(-1, 0)];
    assert_eq!(expected, rotate(&piece_t, 2));
    assert_eq!(expected, rotate(&piece_t, -2));

    let expected = vec![Point(-1, 0), Point(0, 0), Point(1, 0), Point(0, -1)];
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
    let piece_i = build_piece(I);

    let expected = Some(vec![Point(1, 0), Point(1, 1), Point(1, 2), Point(1, 3)]);
    assert_eq!(expected, try_position(&field, &Point(1, 2), &piece_i, 0));

    // note the same as above, but just the order is different
    let expected = Some(vec![Point(1, 3), Point(1, 2), Point(1, 1), Point(1, 0)]);
    assert_eq!(expected, try_position(&field, &Point(1, 2), &piece_i, 2));

    let expected = Some(vec![Point(3, 2), Point(2, 2), Point(1, 2), Point(0, 2)]);
    assert_eq!(expected, try_position(&field, &Point(2, 2), &piece_i, 1));

    let expected = Some(vec![Point(0, 2), Point(1, 2), Point(2, 2), Point(3, 2)]);
    assert_eq!(expected, try_position(&field, &Point(2, 2), &piece_i, 3));

    let expected = None;
    assert_eq!(expected, try_position(&field, &Point(3, 2), &piece_i, 1));
}

pub fn convert_field(_: &str) -> Field {
    // TODO implement this to
    Field {
        cells: vec![],
        height: 0,
        width: 0,
    }
}
