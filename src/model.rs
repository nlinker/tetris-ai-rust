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

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
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

    pub fn clear_current_shape(&mut self) {
        clear_position(&mut self.field, &self.curr_cells);
    }

    pub fn draw_current_shape(&mut self) {
        draw_shape(&mut self.field, &self.curr_cells, self.curr_shape_idx);
    }

    pub fn burn_lines(&mut self) {
        let mut burn_is: Vec<usize> = vec![];
        self.draw_current_shape();
        for i in 0..self.field.height {
            if self.field.cells[i].iter().all(|c| *c != 0) {
                burn_is.push(i);
            }
        }
        // burning makes the cells fall down, therefore we iterate down up
        // `it` skips the rows without burning
        let mut it = self.field.height - 1;
        for i in (0..self.field.height).rev() {
            if it != i {
                for j in 0..self.field.width {
                    self.field.cells[it][j] = self.field.cells[i][j];
                }
            }
            if !burn_is.contains(&i) && it > 0 {
                it -= 1;
            }
        }
        self.score += burn_is.len() as u32;
    }

    fn spawn_next_shape(&mut self) -> () {
        self.curr_shape_idx = self.next_shape_idx;
        self.next_shape_idx = self.rng.gen_range(0, TETRIMINOES.len());
        self.rotation = 0;
        self.base = Point(1, self.field.width as i32 / 2);
        if let Some(cells) = self.try_current_position(&self.base, self.rotation) {
            for i in 0..cells.len() {
                self.curr_cells[i] = cells[i];
            }
        } else {
            self.game_over = true;
        }
    }

    pub fn step(&mut self, action: Action) -> bool {
        if self.game_over {
            return true;
        }
        match action {
            Action::Tick => {
                // clear current
                let base_new = Point(self.base.0 + 1, self.base.1);
                if let Some(cells) = self.try_current_position(&base_new, self.rotation) {
                    self.base = base_new;
                    for i in 0..cells.len() {
                        self.curr_cells[i] = cells[i];
                    }
                } else {
                    self.burn_lines();
                    self.spawn_next_shape();
                }
            }
            Action::HardDrop => {
                let mut i = self.base.0;
                // the last valid position on the path
                let mut cur_cells: Option<Vec<Point>> = None;
                loop {
                    let base_new = Point(i, self.base.1);
                    let cells = self.try_current_position(&base_new, self.rotation);
                    if cells.is_none() {
                        break;
                    }
                    cur_cells = cells;
                    i += 1;
                }
                if let Some(cells) = cur_cells {
                    for i in 0..cells.len() {
                        self.curr_cells[i] = cells[i];
                    }
                    self.base = Point(i, self.base.1);
                    self.burn_lines();
                    self.spawn_next_shape();
                }
            }
            Action::Down => {
                // clear current
                let base_new = Point(self.base.0 + 1, self.base.1);
                if let Some(cells) = self.try_current_position(&base_new, self.rotation) {
                    self.base = base_new;
                    for i in 0..cells.len() {
                        self.curr_cells[i] = cells[i];
                    }
                }
            }
            Action::Left => {
                // clear current
                let base_new = Point(self.base.0, self.base.1 - 1);
                if let Some(cells) = self.try_current_position(&base_new, self.rotation) {
                    self.base = base_new;
                    for i in 0..cells.len() {
                        self.curr_cells[i] = cells[i];
                    }
                }
            }
            Action::Right => {
                // clear current
                let base_new = Point(self.base.0, self.base.1 + 1);
                if let Some(cells) = self.try_current_position(&base_new, self.rotation) {
                    self.base = base_new;
                    for i in 0..cells.len() {
                        self.curr_cells[i] = cells[i];
                    }
                }
            }
            Action::RotateCCW => {
                // clear current
                let rotation_new = self.rotation + 1;
                if let Some(cells) = self.try_current_position(&self.base, rotation_new) {
                    self.rotation = rotation_new;
                    for i in 0..cells.len() {
                        self.curr_cells[i] = cells[i];
                    }
                }
            }
            Action::RotateCW => {
                // clear current
                let rotation_new = self.rotation - 1;
                if let Some(cells) = self.try_current_position(&self.base, rotation_new) {
                    self.rotation = rotation_new;
                    for i in 0..cells.len() {
                        self.curr_cells[i] = cells[i];
                    }
                }
            }
        }
        return false;
    }

    /// The result is similar to (without comment sign)
    /// ```
    /// // . . . .
    /// // . . .#.
    /// // . .#.#.
    /// // .#.#.#.
    /// ```
    pub fn prettify_game_state(&self, rewind: bool, _use_colors: bool) -> String {
        let m = self.field.height;
        let n = self.field.width;
        let mut result = String::with_capacity(m * (2 * n + 1) + 2);
        result.push_str(&String::from_utf8(vec![b' '; 2 * n]).unwrap());
        // in the raw mode we need to to rewind cursor, therefore we prepend \r
        result.push_str("\r\n");
        result.push_str(&format!("score: {}\r\n", self.score));
        result.push_str(&format!("next shape: {}\r\n", &TETRIMINOES[self.next_shape_idx]
            .style.apply_to(self.next_shape_idx.to_string())));
        result.push_str(&format!("current shape: {}\n", &TETRIMINOES[self.curr_shape_idx]
            .style.apply_to(self.curr_shape_idx.to_string())));
        // now put all the stuff
        for i in 0..m {
            for j in 0..n {
                if j == 0 {
                    // intersperse the line with spaces
                    result.push('.');
                }
                let ij = Point(i as i32, j as i32);
                let cell = if self.curr_cells.iter().all(|c| *c != ij) {
                    self.field.cells[i][j]
                } else {
                    self.curr_shape_idx as u8 + 1
                };
                let piece = if cell == 0 {
                    result.push(' ');
                    result.push('.');
                } else {
                    let style: &Style = &TETRIMINOES[cell as usize - 1].style;
                    result.push_str(&style.apply_to('#').to_string());
                    result.push('.');
                };
            }
            // finish the current line
            result.push_str("\r\n");
        }
        if rewind {
            for _ in 0..(m + 5) {
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
