use crate::utils::Trim;

/// `field` is 4x4 field with
/// `ri` and `rj` define rotation point
pub struct RawShape<'a> {
    field: &'a str,
    ri: f32,
    rj: f32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Point(pub i32, pub i32);

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Shape {
    pub diffs: Vec<Point>,
    pub shift: Point,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Color {
    Cyan,
    Blue,
    Orange,
    Yellow,
    Lime,
    Purple,
    Red,
    Grey,
}

/// In the loop `i` runs from `0` to `height-1`; `j` runs from `0` to `width-1`
/// Example:
/// ```rust
/// use tetris::model::Field;
/// let field = Field {
///     cells: vec![
///         vec![0, 0, 0, 0],
///         vec![0, 0, 0, 0],
///         vec![0, 0, 0, 0],
///         vec![0, 0, 0, 0],
///         vec![0, 1, 1, 1],
///     ],
///     height: 5,
///     width: 4,
/// };
/// for i in 0..field.height {
///     for j in 0..field.width {
///         // ok to access field[i][j]
///         println!("{}", field.cells[i][j]);
///     }
/// }
/// ```
#[derive(Debug, Eq, PartialEq)]
pub struct Field {
    pub cells: Vec<Vec<u8>>,
    pub height: usize,
    pub width: usize,
}


pub const I: RawShape<'static> = RawShape {
    field: r#"
        . . . .
        * * * *
        . . . .
        . . . .
    "#,
    ri: 1.0,
    rj: 1.5,
};
pub const O: RawShape<'static> = RawShape {
    field: r#"
        . . . .
        . * * .
        . * * .
        . . . .
    "#,
    ri: 1.5,
    rj: 1.5,
};
pub const L: RawShape<'static> = RawShape {
    field: r#"
        . * . .
        . * . .
        . * * .
        . . . .
    "#,
    ri: 1.0,
    rj: 1.5,
};
pub const J: RawShape<'static> = RawShape {
    field: r#"
        . . * .
        . . * .
        . * * .
        . . . .
    "#,
    ri: 1.0,
    rj: 1.5,
};
pub const T: RawShape<'static> = RawShape {
    field: r#"
        . . . .
        * * * .
        . * . .
        . . . .
    "#,
    ri: 1.5,
    rj: 1.0,
};
pub const S: RawShape<'static> = RawShape {
    field: r#"
        . . . .
        . * * .
        * * . .
        . . . .
    "#,
    ri: 1.5,
    rj: 1.0,
};
pub const Z: RawShape<'static>  = RawShape {
    field: r#"
        . . . .
        * * . .
        . * * .
        . . . .
    "#,
    ri: 1.5,
    rj: 1.0,
};


/// return the shape points relative of (0, 0) with parity
pub fn build_piece(src: RawShape<'_>) -> Shape {
    let mut diffs: Vec<Point> = Vec::with_capacity(4);
    let mut ci = 0;
    // shift is needed to know how to round the shape after the rotation
    let mut shift_i = 0;
    let mut shift_j = 0;
    for line in src.field.trim_indent().split('\n') {
        let mut cj = 0;
        for c in line.chars() {
            if c == '*' {
                let i = (2.0 * ((ci as f32) - src.ri)).trunc() as i32;
                let j = (2.0 * ((cj as f32) - src.rj)).trunc() as i32;
                diffs.push(Point(i, j));
                if i % 2 == 1 { shift_i = 1; }
                if j % 2 == 1 { shift_j = 1; }
                cj += 1;
            } else if c == '.' {
                cj += 1;
            }
        }
        ci += 1;
    }
    Shape { diffs, shift: Point(shift_i, shift_j) }
}

pub fn rotate(shape: &Shape, r: i8) -> Vec<Point> {
    // modulo, NOT the remainder, see https://stackoverflow.com/a/41422009/5066426
    let r = (r % 4 + 4) % 4;
    let mut p = shape.clone();
    for _ in 0..r {
        for d in &mut p.diffs {
            // rotate counterclockwise (-1, -2) => (2, -1),
            // i.e. negate the second coordinate and then swap
            let t = -d.1;
            d.1 = d.0;
            d.0 = t;
        }
        // swap shift
        let t = p.shift.1;
        p.shift.1 = p.shift.0;
        p.shift.0 = t;
    }
    // shift and divide, so (0, 0) is the integer center of the rotated shape
    for d in &mut p.diffs {
        d.0 = (d.0 - p.shift.0) / 2;
        d.1 = (d.1 - p.shift.1) / 2;
    }
    p.diffs
}

pub fn try_position(field: &Field, base: &Point, shape: &Shape, r: i8) -> Option<Vec<Point>> {
    let mut points = rotate(&shape, r);
    for d in &points {
        let i = base.0 + d.0;
        let j = base.1 + d.1;
        println!("{:?}", (i, j));
        if i < 0 || field.height as i32 <= i {
            return None;
        } else if j < 0 || field.width as i32 <= j {
            return None;
        } else if field.cells[i as usize][j as usize] != 0 {
            return None;
        }
    }
    // shift points w.r.t. base
    for d in &mut points {
        d.0 = base.0 + d.0;
        d.1 = base.1 + d.1;
    }
    Some(points)
}


//
//// shapes, and the rotation point
//lazy_static! {
//    static ref SHAPES: [Shape; 7] = { ... };
//}
