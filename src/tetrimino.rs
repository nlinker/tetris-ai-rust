
use crate::utils::Trim;
use crate::model::Point;
use termion::color::Color;
use termion::{color, style};
use std::borrow::Borrow;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Style {
    Empty,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

//impl<T> Color for Box<T> where T: Color + ?Sized {
//    fn write_t(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
//        unimplemented!()
//    }
//
//    fn write_bg(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
//        unimplemented!()
//    }
//}

impl Style {
    pub fn apply_to(&self, block: &str) -> String {
        let fg: Box<dyn Color> = match self {
            Style::Empty   => Box::new(color::Reset),
            Style::Black   => Box::new(color::LightBlack),
            Style::Red     => Box::new(color::LightRed),
            Style::Green   => Box::new(color::LightGreen),
            Style::Yellow  => Box::new(color::LightYellow),
            Style::Blue    => Box::new(color::LightBlue),
            Style::Magenta => Box::new(color::LightMagenta),
            Style::Cyan    => Box::new(color::LightCyan),
            Style::White   => Box::new(color::LightWhite),
        };
        format!("{}{}{}", color::Fg(fg.borrow()), block, style::Reset)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Tetrimino {
    pub diffs: Vec<Point>,
    pub shift: Point,
    pub style: Style,
}

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
    pub static ref TETRIMINOES: [Tetrimino; 7] = [
        build_tetrimino(I),
        build_tetrimino(O),
        build_tetrimino(T),
        build_tetrimino(S),
        build_tetrimino(Z),
        build_tetrimino(J),
        build_tetrimino(L),
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
    color: "cyan",
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
    color: "yellow",
};
pub const T: RawShape<'static> = RawShape {
    field: r#"
        . * . .
        * * * .
        . . . .
        . . . .
    "#,
    ri: 1.0,
    rj: 1.0,
    color: "magenta",
};
pub const S: RawShape<'static> = RawShape {
    field: r#"
        . * * .
        * * . .
        . . . .
        . . . .
    "#,
    ri: 1.0,
    rj: 1.0,
    color: "green",
};
pub const Z: RawShape<'static>  = RawShape {
    field: r#"
        * * . .
        . * * .
        . . . .
        . . . .
    "#,
    ri: 1.0,
    rj: 1.0,
    color: "red",
};
pub const J: RawShape<'static> = RawShape {
    field: r#"
        * . . .
        * * * .
        . . . .
        . . . .
    "#,
    ri: 1.0,
    rj: 1.0,
    color: "blue",
};
pub const L: RawShape<'static> = RawShape {
    field: r#"
        . . *.
        * * * .
        . . . .
        . . . .
    "#,
    ri: 1.0,
    rj: 1.0,
    color: "white",
};

/// return the shape points relative of (0, 0) with parity
pub fn build_tetrimino(src: RawShape<'_>) -> Tetrimino {
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
    let color = color_from_str(src.color);
    Tetrimino { diffs, shift: Point(shift_i, shift_j), style: color }
}

pub fn color_from_str(color_str: &str) -> Style {
    match color_str {
        "black"   => Style::Black,
        "red"     => Style::Red,
        "green"   => Style::Green,
        "yellow"  => Style::Yellow,
        "blue"    => Style::Blue,
        "magenta" => Style::Magenta,
        "cyan"    => Style::Cyan,
        "white"   => Style::White,
        _ => unreachable!(),
    }
}