use crate::utils::Trim;

/// `field` is 4x4 field with
/// `ri` and `rj` define rotation point
pub struct RawPiece<'a> {
    field: &'a str,
    ri: f32,
    rj: f32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Point(pub i32, pub i32);

pub const I: RawPiece<'static> = RawPiece {
    field: r#"
        . . . .
        * * * *
        . . . .
        . . . .
    "#,
    ri: 1.0,
    rj: 1.5,
};
pub const O: RawPiece<'static> = RawPiece {
    field: r#"
        . . . .
        . * * .
        . * * .
        . . . .
    "#,
    ri: 1.5,
    rj: 1.5,
};
pub const L: RawPiece<'static> = RawPiece {
    field: r#"
        . * . .
        . * . .
        . * * .
        . . . .
    "#,
    ri: 1.0,
    rj: 1.5,
};
pub const J: RawPiece<'static> = RawPiece {
    field: r#"
        . . * .
        . . * .
        . * * .
        . . . .
    "#,
    ri: 1.0,
    rj: 1.5,
};
pub const T: RawPiece<'static> = RawPiece {
    field: r#"
        . . . .
        * * * .
        . * . .
        . . . .
    "#,
    ri: 1.5,
    rj: 1.0,
};
pub const S: RawPiece<'static> = RawPiece {
    field: r#"
        . . . .
        . * * .
        * * . .
        . . . .
    "#,
    ri: 1.5,
    rj: 1.0,
};
pub const Z: RawPiece<'static>  = RawPiece {
    field: r#"
        . . . .
        * * . .
        . * * .
        . . . .
    "#,
    ri: 1.5,
    rj: 1.0,
};


/// return the piece points relative of (0, 0) with parity
pub fn convert(src: RawPiece<'_>) -> (Vec<Point>, Point) {
    let mut vec: Vec<Point> = Vec::with_capacity(4);
    let mut ci = 0;
    // parity is needed to know how to round the piece after the rotation
    let mut parity_i = 0;
    let mut parity_j = 0;
    for line in src.field.trim_indent().split('\n') {
        let mut cj = 0;
        for c in line.chars() {
            if c == '*' {
                let i = (2.0 * ((ci as f32) - src.ri)).trunc() as i32;
                let j = (2.0 * ((cj as f32) - src.rj)).trunc() as i32;
                vec.push(Point(i, j));
                if i % 2 == 1 { parity_i = 1; }
                if j % 2 == 1 { parity_j = 1; }
                cj += 1;
            } else if c == '.' {
                cj += 1;
            }
        }
        ci += 1;
    }
    (vec, Point(parity_i, parity_j))
}


//struct Grid {
//    grid: Vec<Vec<Cell>>,
//}

//struct Piece {
//    template: Vec<i8>, // i8 == -128..127
//    // rotation coordinates
//    center_x: f32,
//    center_y: f32,
//}
//
//// pieces, and the rotation point
//lazy_static! {
//    static ref PIECES: Vec<Piece> = {
//        let mut v: Vec<Piece> = Vec::with_capacity(10);
//        v
//    };
//}
