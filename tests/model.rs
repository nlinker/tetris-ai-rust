#![feature(type_ascription)]

//use std::collections::HashSet;
use tetris::model::{Point, convert, I, O, L, J, T, S, Z, Piece};

#[test]
fn test_conversion() {
    let diffs = vec![Point(0, -3), Point(0, -1), Point(0, 1), Point(0, 3)]: Vec<Point>;
    let shift = Point(0, 1);
    assert_eq!(Piece { diffs, shift }, convert(I));

    let diffs = vec![Point(-1, -1), Point(-1, 1), Point(1, -1), Point(1, 1)]: Vec<Point>;
    let shift = Point(1, 1);
    assert_eq!(Piece { diffs, shift }, convert(O));

    let diffs = vec![Point(-2, -1), Point(0, -1), Point(2, -1), Point(2, 1)]: Vec<Point>;
    let shift = Point(0, 1);
    assert_eq!(Piece { diffs, shift }, convert(L));

    let diffs = vec![Point(-2, 1), Point(0, 1), Point(2, -1), Point(2, 1)]: Vec<Point>;
    let shift = Point(0, 1);
    assert_eq!(Piece { diffs, shift }, convert(J));

    let diffs = vec![Point(-1, -2), Point(-1, 0), Point(-1, 2), Point(1, 0)]: Vec<Point>;
    let shift = Point(1, 0);
    assert_eq!(Piece { diffs, shift }, convert(T));

    let diffs = vec![Point(-1, 0), Point(-1, 2), Point(1, -2), Point(1, 0)]: Vec<Point>;
    let shift = Point(1, 0);
    assert_eq!(Piece { diffs, shift }, convert(S));

    let diffs = vec![Point(-1, -2), Point(-1, 0), Point(1, 0), Point(1, 2)]: Vec<Point>;
    let shift = Point(1, 0);
    assert_eq!(Piece { diffs, shift }, convert(Z));
}