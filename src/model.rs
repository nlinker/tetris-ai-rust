use crate::utils::Trim;
use lazy_static;
use console::Style;
use rand_xoshiro::Xoroshiro128StarStar;
use rand::{SeedableRng, Rng};
use std::fmt;

/// `field` is 4x4 field with
/// `ri` and `rj` define rotation point
/// `color` must correspond to
pub struct RawShape<'a> {
    field: &'a str,
    ri: f32,
    rj: f32,
    color: &'a str,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Point(pub i32, pub i32);

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Shape {
    pub diffs: Vec<Point>,
    pub shift: Point,
    pub style: Style,
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
///         // field[i][j] == 0 corresponds to empty
///         // field[i][j] == 1..7 corresponds to [I, O, L, J, T, S, Z], see SHAPES
///         println!("{}", field.cells[i][j]);
///     }
/// }
/// ```
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Field {
    pub cells: Vec<Vec<u8>>,
    pub height: usize,
    pub width: usize,
}

pub enum Action {
    Left,
    Right,
    Down,
    RotateCW,  // clockwise
    RotateCCW, // counterclockwise
    Tick,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct GameState {
    field: Field,
    base: Point,
    rotation: i32,
    curr_cells: Vec<Point>,
    curr_shape_idx: usize,
    next_shape_idx: usize,
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
    color: "red",
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
    color: "green",
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
    color: "yellow",
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
    color: "blue",
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
    color: "magenta",
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
    color: "cyan",
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
    color: "white",
};

impl GameState {
    pub fn initial(height: usize, width: usize, seed: Option<u64>) -> GameState {
        let mut random = if let Some(seed) = seed {
            Xoroshiro128StarStar::seed_from_u64(seed)
        } else {
            Xoroshiro128StarStar::from_entropy()
        };
        let mut field = Field {
            cells: vec![vec![0; width]; height],
            height,
            width,
        };
        let curr_shape_idx = random.gen_range(0, SHAPES.len());
        let next_shape_idx = random.gen_range(0, SHAPES.len());
        let base = Point(1, width as i32 / 2);
        let rotation = 0;
        if let Some(curr_points) = try_position(&field, &base, &SHAPES[curr_shape_idx], 0) {
            put_position(&mut field, curr_shape_idx, &curr_points[..]);
            GameState {
                field,
                base,
                rotation,
                curr_cells: curr_points,
                curr_shape_idx,
                next_shape_idx,
            }
        } else {
            panic!("Impossible initial state")
        }
    }


    pub fn step(&mut self, action: Action) {
        match action {
            Tick => {
                // clear current
                // try the same base.1 + 1
                // draw current
            },
            _ => unreachable!("implement this"),
        }
    }

    pub fn prettify_game_state(&self, rewind: bool, _use_colors: bool) -> String {
        let m = self.field.height;
        let n = self.field.width;
        let mut result = String::with_capacity(m * (2 * n + 1) + 2);
        // now put all the stuff
        let empty_style = Style::new();
        let mut current_piece: String = String::with_capacity(n * 4);
        let mut current_style = &empty_style;
        let mut prev_symbol: Option<char> = None;
        let mut current_symbol: Option<char> = None;

        result.push_str(&format!("current shape: {}\n", &SHAPES[self.curr_shape_idx]
            .style.apply_to(self.curr_shape_idx.to_string())));
        result.push_str(&format!("next shape: {}\n", &SHAPES[self.next_shape_idx]
            .style.apply_to(self.next_shape_idx.to_string())));
        for i in 0..m {
            for j in 0..n {
                // intersperse the line with spaces
                if j != 0 {
                    current_piece.push(' ');
                }

                current_symbol = if self.field.cells[i][j] == 0 { Some('.') } else { Some('*') };
                if current_symbol != prev_symbol {
                    result.push_str(&current_style.apply_to(&current_piece).to_string());
                    current_piece.clear();
                }
                match self.field.cells[i][j] {
                    0 => current_style = &empty_style,
                    k => current_style = &SHAPES[k as usize - 1].style
                }
                if let Some(c) = current_symbol {
                    current_piece.push(c);
                }
                prev_symbol = current_symbol;
            }
            current_piece.push('\n');
            result.push_str(&current_style.apply_to(&current_piece).to_string());
            current_piece.clear();
        }
        if rewind {
            for k in 0..(m + 2) {
                result.push_str("\x1B[A") // up
            }
        }
        result
    }
}

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

pub fn try_position(field: &Field, base: &Point, shape: &Shape, r: i8) -> Option<Vec<Point>> {
    let mut points = rotate(&shape, r);
    for d in &points {
        let i = base.0 + d.0;
        let j = base.1 + d.1;
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

pub fn put_position(field: &mut Field, shape_idx: usize, points: &[Point]) {
    for p in points {
        field.cells[p.0 as usize][p.1 as usize] = (shape_idx + 1) as u8;
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str(&self.prettify_game_state(false, false));
        Ok(())
    }
}
