use console::Style;
use rand_xoshiro::Xoroshiro128StarStar;
use rand::{SeedableRng, Rng};
use std::fmt;
use crate::tetrimino::TETRIMINOES;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Point(pub i32, pub i32);

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Tetrimino {
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
///         // field[i][j] == 1..7 corresponds to [I, O, L, J, T, S, Z], see TETRIMINOES
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
    HardDrop,
    Tick,
}

/// TODO How do I make `rng` serializable?
/// Essentially it is two u64 numbers, but they are private...
#[derive(Debug, Clone)]
pub struct GameState {
    field: Field,
    game_over: bool,
    base: Point,
    rotation: i32,
    curr_cells: Vec<Point>,
    curr_shape_idx: usize,
    next_shape_idx: usize,
    score: u32,
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
        let curr_shape_idx = rng.gen_range(0, TETRIMINOES.len());
        let next_shape_idx = rng.gen_range(0, TETRIMINOES.len());
        let base = Point(1, width as i32 / 2);
        let rotation = 0;
        if let Some(curr_cells) = try_position(&field, &base, &TETRIMINOES[curr_shape_idx], 0) {
            draw_shape(&mut field, &curr_cells[..], curr_shape_idx);
            GameState {
                field,
                game_over: false,
                base,
                rotation,
                curr_cells,
                curr_shape_idx,
                next_shape_idx,
                score: 0,
                rng,
            }
        } else {
            GameState {
                field,
                game_over: true,
                base,
                rotation,
                curr_cells: vec![],
                curr_shape_idx,
                next_shape_idx,
                score: 0,
                rng,
            }
        }
    }

    pub fn try_current_position(&self, base: &Point, rotation: i32) -> Option<Vec<Point>>{
        let shape = &TETRIMINOES[self.curr_shape_idx];
        try_position(&self.field, &base, &shape, rotation)
    }

    pub fn clear_current_position(&mut self) {
        clear_position(&mut self.field, &self.curr_cells);
    }

    pub fn draw_current_shape(&mut self) {
        draw_shape(&mut self.field, &self.curr_cells, self.curr_shape_idx);
    }

    pub fn step(&mut self, action: Action) {
        match action {
            Action::Tick => {
                // clear current
                let base_new = Point(self.base.0 + 1, self.base.1);
                self.clear_current_position();
                if let Some(cells) = self.try_current_position(&base_new, self.rotation) {
                    self.base = base_new;
                    for i in 0..cells.len() {
                        self.curr_cells[i] = cells[i];
                    }
                    self.draw_current_shape();
                } else {
                    self.draw_current_shape();
                    // TODO burn lines
                    self.curr_shape_idx = self.next_shape_idx;
                    self.next_shape_idx = self.rng.gen_range(0, TETRIMINOES.len());
                    self.rotation = 0;
                    self.base = Point(1, self.field.width as i32 / 2);
                    if let Some(cells) = self.try_current_position(&self.base, self.rotation) {
                        for i in 0..cells.len() {
                            self.curr_cells[i] = cells[i];
                        }
                        self.draw_current_shape();
                    } else {
                        // TODO end of game
                        panic!(self.prettify_game_state(false, false));
                    }
                }
            },
            Action::Left => {
                // clear current
                let base_new = Point(self.base.0, self.base.1 - 1);
                self.clear_current_position();
                if let Some(cells) = self.try_current_position(&base_new, self.rotation) {
                    self.base = base_new;
                    for i in 0..cells.len() {
                        self.curr_cells[i] = cells[i];
                    }
                }
                self.draw_current_shape();
            },
            Action::Right => {
                // clear current
                let base_new = Point(self.base.0, self.base.1 + 1);
                self.clear_current_position();
                if let Some(cells) = self.try_current_position(&base_new, self.rotation) {
                    self.base = base_new;
                    for i in 0..cells.len() {
                        self.curr_cells[i] = cells[i];
                    }
                }
                self.draw_current_shape();
            },
            Action::RotateCCW => {
                // clear current
                let rotation_new = self.rotation + 1;
                self.clear_current_position();
                if let Some(cells) = self.try_current_position(&self.base, rotation_new) {
                    self.rotation = rotation_new;
                    for i in 0..cells.len() {
                        self.curr_cells[i] = cells[i];
                    }
                }
                self.draw_current_shape();
            }
            Action::RotateCW => {
                // clear current
                let rotation_new = self.rotation - 1;
                self.clear_current_position();
                if let Some(cells) = self.try_current_position(&self.base, rotation_new) {
                    self.rotation = rotation_new;
                    for i in 0..cells.len() {
                        self.curr_cells[i] = cells[i];
                    }
                }
                self.draw_current_shape();
            }
            _ => unreachable!("implement this"),
        }
    }

    pub fn prettify_game_state(&self, rewind: bool, _use_colors: bool) -> String {
        let m = self.field.height;
        let n = self.field.width;
        let mut result = String::with_capacity(m * (2 * n + 1) + 2);
        result.push_str(&format!("current shape: {}\n", &TETRIMINOES[self.curr_shape_idx]
            .style.apply_to(self.curr_shape_idx.to_string())));
        result.push_str(&format!("next shape: {}\n", &TETRIMINOES[self.next_shape_idx]
            .style.apply_to(self.next_shape_idx.to_string())));
        // now put all the stuff
        let empty_style = Style::new();
        let mut curr_piece: String = String::with_capacity(n * 4);

        for i in 0..m {
            curr_piece.clear();
            let mut curr_style = &empty_style;
            let mut prev_symbol: Option<u8> = None;
            let mut curr_symbol: Option<u8>;
            for j in 0..n {
                // |000|1111|22222|33|
                // ^   ^    ^     ^  ^
                // each line subdivides by groups, and on each boundary we
                // output the previous group and calculate the style
                if j != 0 {
                    // intersperse the line with spaces
                    curr_piece.push(' ');
                }
                let cell = self.field.cells[i][j];
                curr_symbol = Some(cell);

                if curr_symbol != prev_symbol {
                    // a boundary found
                    if prev_symbol.is_some() {
                        result.push_str(&curr_style.apply_to(&curr_piece).to_string());
                        curr_piece.clear();
                    }
                    curr_style = if cell == 0 { &empty_style } else { &TETRIMINOES[cell as usize - 1].style };
                }

                curr_piece.push(if cell == 0 { '.' } else { '#' });
                prev_symbol = curr_symbol;
            }
            // finish the current line
            result.push_str(&curr_style.apply_to(&curr_piece).to_string());
            result.push('\n');
        }
        if rewind {
            for _ in 0..(m + 3) {
                result.push_str("\x1B[A") // up
            }
        }
        result
    }
}

pub fn try_position(field: &Field, base: &Point, shape: &Tetrimino, r: i32) -> Option<Vec<Point>> {
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

pub fn rotate(shape: &Tetrimino, r: i32) -> Vec<Point> {
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

pub fn draw_shape(field: &mut Field, points: &[Point], shape_idx: usize) {
    for p in points {
        field.cells[p.0 as usize][p.1 as usize] = (shape_idx + 1) as u8;
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str(&self.prettify_game_state(false, false))?;
        Ok(())
    }
}
