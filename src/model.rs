use console::Style;
use rand_xoshiro::Xoroshiro128StarStar;
use rand::{SeedableRng, Rng, RngCore};
use std::fmt;
use crate::shapes::{SHAPES, build_shape, I};

fn x() {
    build_shape(I);
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
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

#[derive(Debug, Clone)]
pub struct GameState {
    field: Field,
    base: Point,
    rotation: i32,
    curr_cells: Vec<Point>,
    curr_shape_idx: usize,
    next_shape_idx: usize,
    rng: Xoroshiro128StarStar,
}

impl GameState {
    pub fn initial(height: usize, width: usize, seed: Option<u64>) -> GameState {
        let mut rng = if let Some(seed) = seed {
            Xoroshiro128StarStar::seed_from_u64(seed)
        } else {
            Xoroshiro128StarStar::from_entropy()
        };
        let mut field = Field {
            cells: vec![vec![0; width]; height],
            height,
            width,
        };
        let curr_shape_idx = rng.gen_range(0, SHAPES.len());
        let next_shape_idx = rng.gen_range(0, SHAPES.len());
        let base = Point(1, width as i32 / 2);
        let rotation = 0;
        if let Some(curr_points) = try_position(&field, &base, &SHAPES[curr_shape_idx], 0) {
            put_position(&mut field, &curr_points[..], curr_shape_idx);
            GameState {
                field,
                base,
                rotation,
                curr_cells: curr_points,
                curr_shape_idx,
                next_shape_idx,
                rng,
            }
        } else {
            panic!("Impossible initial state")
        }
    }

    pub fn step(&mut self, action: Action) {
        match action {
            Action::Tick => {
                // clear current
                clear_position(&mut self.field, &self.curr_cells);
                let base_new = Point(self.base.0 + 1, self.base.1);
                let shape = &SHAPES[self.curr_shape_idx];
                if let Some(cells) = try_position(&self.field, &base_new, &shape, self.rotation) {
                    self.base = base_new;
                    for i in 0..cells.len() {
                        self.curr_cells[i] = cells[i];
                    }
                    put_position(&mut self.field, &self.curr_cells, self.curr_shape_idx);
                } else {
                    put_position(&mut self.field, &self.curr_cells, self.curr_shape_idx);
                    self.curr_shape_idx = self.next_shape_idx;
                    self.next_shape_idx = self.rng.gen_range(0, SHAPES.len());
                    self.base = Point(2, self.field.width as i32 / 2);
                    let shape = &SHAPES[self.curr_shape_idx];
                    let cells = try_position(&self.field, &self.base, &shape, self.rotation).unwrap();
                    for i in 0..cells.len() {
                        self.curr_cells[i] = cells[i];
                    }
                    put_position(&mut self.field, &self.curr_cells, self.curr_shape_idx);
                }

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
            for _ in 0..(m + 3) {
                result.push_str("\x1B[A") // up
            }
        }
        result
    }
}

pub fn try_position(field: &Field, base: &Point, shape: &Shape, r: i32) -> Option<Vec<Point>> {
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

pub fn rotate(shape: &Shape, r: i32) -> Vec<Point> {
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

pub fn clear_position(field: &mut Field, points: &[Point]) {
    for p in points {
        field.cells[p.0 as usize][p.1 as usize] = 0;
    }
}

pub fn put_position(field: &mut Field, points: &[Point], shape_idx: usize) {
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
