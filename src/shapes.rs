
use console::Style;
use crate::utils::Trim;
use crate::model::{Shape, Point};

/// `field` is 4x4 field with
/// `ri` and `rj` define rotation point
/// `color` must correspond to
pub struct RawShape<'a> {
    field: &'a str,
    ri: f32,
    rj: f32,
    color: &'a str,
}

lazy_static! {
    pub static ref SHAPES: [Shape; 7] = [
        build_shape(I),
        build_shape(O),
        build_shape(L),
        build_shape(J),
        build_shape(T),
        build_shape(S),
        build_shape(Z),
    ];
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
    color: "cyan.bold",
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
    color: "yellow.bold",
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
    color: "magenta.bold",
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
    color: "green.bold",
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
    color: "red.bold",
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
    color: "blue.bold",
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
    color: "white.bold",
};

/// return the shape points relative of (0, 0) with parity
pub fn build_shape(src: RawShape<'_>) -> Shape {
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
    let style = Style::from_dotted_str(src.color);
    Shape { diffs, shift: Point(shift_i, shift_j), style }
}
