#![feature(type_ascription)]

//use std::collections::HashSet;
use tetris::model::{Point, convert, I, O, L, J, T, S, Z};

#[test]
fn test_conversion() {
    let xs = vec![Point(0, -3), Point(0, -1), Point(0, 1), Point(0, 3)]: Vec<Point>;
    assert_eq!((xs, Point(0, 1)), convert(I));

    let xs = vec![Point(-1, -1), Point(-1, 1), Point(1, -1), Point(1, 1)]: Vec<Point>;
    assert_eq!((xs, Point(1, 1)), convert(O));

    let xs = vec![Point(-2, -1), Point(0, -1), Point(2, -1), Point(2, 1)]: Vec<Point>;
    assert_eq!((xs, Point(0, 1)), convert(L));

    let xs = vec![Point(-2, 1), Point(0, 1), Point(2, -1), Point(2, 1)]: Vec<Point>;
    assert_eq!((xs, Point(0, 1)), convert(J));

    let xs = vec![Point(-1, -2), Point(-1, 0), Point(-1, 2), Point(1, 0)]: Vec<Point>;
    assert_eq!((xs, Point(1, 0)), convert(T));

    let xs = vec![Point(-1, 0), Point(-1, 2), Point(1, -2), Point(1, 0)]: Vec<Point>;
    assert_eq!((xs, Point(1, 0)), convert(S));

    let xs = vec![Point(-1, -2), Point(-1, 0), Point(1, 0), Point(1, 2)]: Vec<Point>;
    assert_eq!((xs, Point(1, 0)), convert(Z));
}