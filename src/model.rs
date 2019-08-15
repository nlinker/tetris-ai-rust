use lazy_static;
use std::collections::HashMap;
use std::fmt;
use rand_xoshiro::Xoshiro512StarStar;
use rand::{SeedableRng, Rng};
use rand::prelude::SliceRandom;
use crate::tetrimino::{TETRIMINOES, Style, Tetrimino};
use crate::config::{Config, Scoring, Randomness};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Point(pub i32, pub i32);

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
    pub config: Config,
    pub field: Field,
    pub game_over: bool,
    pub base: Point,
    pub rotation: i8,
    pub curr_cells: Vec<Point>,
    pub curr_shape_idx: usize,
    pub next_shape_idx: usize,
    pub score: u32,
    pub rng: Xoshiro512StarStar,
    pub rng_queue: Vec<usize>,
}


lazy_static!{
    // In each 0th element in each row is the index `(rotation_from, rotation_to)`
    // the coordinates are Descartes, the data are taken from
    // https://tetris.fandom.com/wiki/SRS, section Wall Kicks
    // we convert the tables to the maps `(rotation_from, rotation_to) -> [offset1,...,offset5]`
    // rotations:
    //      0 - without rotation,
    //      1 - pi/2 clockwise,
    //      2 - pi clockwise,
    //      3 - 3pi/2 clockwise = pi/2 counterclockwise

    /// shifts to test for I
    pub static ref WALL_KICKS_I: HashMap<(i8, i8), Vec<Point>> = {
        let table = [
            [(0, 1), (0, 0), (-2, 0), ( 1, 0), (-2,-1), ( 1, 2)],
            [(1, 0), (0, 0), ( 2, 0), (-1, 0), ( 2, 1), (-1,-2)],
            [(1, 2), (0, 0), (-1, 0), ( 2, 0), (-1, 2), ( 2,-1)],
            [(2, 1), (0, 0), ( 1, 0), (-2, 0), ( 1,-2), (-2, 1)],
            [(2, 3), (0, 0), ( 2, 0), (-1, 0), ( 2, 1), (-1,-2)],
            [(3, 2), (0, 0), (-2, 0), ( 1, 0), (-2,-1), ( 1, 2)],
            [(3, 0), (0, 0), ( 1, 0), (-2, 0), ( 1,-2), (-2, 1)],
            [(0, 3), (0, 0), (-1, 0), ( 2, 0), (-1, 2), ( 2,-1)],
        ];
        let mut hm: HashMap<(i8, i8), Vec<_>> = HashMap::new();
        for row in table.iter() {
            hm.insert(row[0],
                row[1..5].iter().map(|xy| Point(-xy.1 as i32, xy.0 as i32)).collect()
            );
        }
        hm
    };
    /// shifts to test for J, L, S, T, Z
    pub static ref WALL_KICKS_X: HashMap<(i8, i8), Vec<Point>> = {
        let table = [
            [(0, 1), (0, 0), (-1, 0), (-1, 1), (0,-2), (-1,-2)],
            [(1, 0), (0, 0), ( 1, 0), ( 1,-1), (0, 2), ( 1, 2)],
            [(1, 2), (0, 0), ( 1, 0), ( 1,-1), (0, 2), ( 1, 2)],
            [(2, 1), (0, 0), (-1, 0), (-1, 1), (0,-2), (-1,-2)],
            [(2, 3), (0, 0), ( 1, 0), ( 1, 1), (0,-2), ( 1,-2)],
            [(3, 2), (0, 0), (-1, 0), (-1,-1), (0, 2), (-1, 2)],
            [(3, 0), (0, 0), (-1, 0), (-1,-1), (0, 2), (-1, 2)],
            [(0, 3), (0, 0), ( 1, 0), ( 1, 1), (0,-2), ( 1,-2)],
        ];
        let mut hm: HashMap<(i8, i8), Vec<_>> = HashMap::new();
        for row in table.iter() {
            hm.insert(row[0],
                row[1..5].iter().map(|xy| Point(-xy.1 as i32, xy.0 as i32)).collect()
            );
        }
        hm
    };
}

impl GameState {
    pub fn initial(height: usize, width: usize, config: Config, seed: Option<u64>) -> GameState {
        let mut rng = if let Some(seed) = seed {
            Xoshiro512StarStar::seed_from_u64(seed)
        } else {
            Xoshiro512StarStar::from_entropy()
        };
        let field = Field {
            cells: vec![vec![0; width]; height],
            height,
            width,
        };
        let mut gs = GameState {
            config,
            field,
            game_over: false,
            base: Point(0, 0),
            rotation: 0,
            curr_cells: Vec::new(),
            curr_shape_idx: 0,
            next_shape_idx: 0,
            score: 0,
            rng,
            rng_queue: Vec::new(),
        };
        gs.reset();
        gs
    }

    /// mutate the game state to the initial one,
    /// we don't reset random generators, but reset random queue
    pub fn reset(&mut self) {
        let curr_shape_idx;
        let next_shape_idx;
        let mut rng_queue;
        match self.config.randomness {
            Randomness::JustRandom => {
                next_shape_idx = self.rng.gen_range(0, TETRIMINOES.len());
                curr_shape_idx = self.rng.gen_range(0, TETRIMINOES.len());
                rng_queue = Vec::new();
            },
            Randomness::ShuffledQueue => {
                rng_queue = (0..TETRIMINOES.len()).collect::<Vec<_>>();
                rng_queue.shuffle(&mut self.rng);
                next_shape_idx = rng_queue.pop().unwrap(); // guaranteed to exist
                curr_shape_idx = rng_queue.pop().unwrap(); // guaranteed to exist
            },
        }
        for i in 0..self.field.height {
            for j in 0..self.field.width {
                self.field.cells[i][j] = 0;
            }
        }
        self.curr_shape_idx = curr_shape_idx;
        self.next_shape_idx = next_shape_idx;
        self.rng_queue = rng_queue;
        // TODO deduplicate with spawn_new_shape function
        self.base = if self.curr_shape_idx == 0 {
            // if it is horizontal I shape
            Point(0, self.field.width as i32 / 2)
        } else {
            Point(1, self.field.width as i32 / 2)
        };
        self.rotation = 0;
        self.score = 0;
        let shape = &TETRIMINOES[curr_shape_idx];
        if let Some(curr_cells) = try_shape(&self.field, &self.base, self.rotation, shape) {
            self.curr_cells = curr_cells;
            self.game_over = false;
        } else {
            self.curr_cells = Vec::new();
            self.game_over = true;
        }
    }

    /// transition should be the pair of `(p, q)`, where `p, q \in {0, 1, 2, 3}`
    pub fn wall_kick_current_shape(&self, transition: (i8, i8)) -> Option<(Point, Vec<Point>)> {
        let test_points: &[Point] = if self.curr_shape_idx == 0 {
            &WALL_KICKS_I.get(&transition)?
        } else {
            &WALL_KICKS_X.get(&transition)?
        };
        // return the first test point, that enables the rotation around
        test_points.into_iter().find_map(|t| {
            let base_new = Point(self.base.0 + t.0, self.base.1 + t.1);
            self.try_current_shape(&base_new, transition.1)
                .map(|v| (base_new, v))
        })
    }

    pub fn try_current_shape(&self, base: &Point, rotation: i8) -> Option<Vec<Point>> {
        let shape = &TETRIMINOES[self.curr_shape_idx];
        try_shape(&self.field, &base, rotation, &shape)
    }

    pub fn draw_current_shape(&mut self) {
        for p in &self.curr_cells {
            self.field.cells[p.0 as usize][p.1 as usize] = (self.curr_shape_idx + 1) as u8;
        }
    }

    pub fn burn_lines(&mut self) -> usize {
        let mut burn_is: Vec<usize> = vec![];
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
        // the rest of lines are populated by zeros
        for i in (0..it + 1).rev() {
            for j in 0..self.field.width {
                self.field.cells[i][j] = 0;
            }
        }
        self.score += match &self.config.scoring {
            Scoring::BurnOnly => 1,
            Scoring::PieceAndBurn => {
                let lines_cleared = burn_is.len() as u32;
                1 + lines_cleared * lines_cleared * (self.field.width as u32)
            },
        };
        burn_is.len()
    }

    pub fn spawn_next_shape(&mut self) {
        let prev_shape_idx = self.curr_shape_idx;
        self.curr_shape_idx = self.next_shape_idx;
        self.next_shape_idx = match &self.config.randomness {
            Randomness::JustRandom => self.rng.gen_range(0, TETRIMINOES.len()),
            Randomness::ShuffledQueue => {
                match self.rng_queue.pop() {
                    Some(idx) => idx,
                    None => {
                        // the rng_queue has been fully depleted
                        for i in 0..TETRIMINOES.len() {
                            self.rng_queue.push(i);
                        }
                        self.rng_queue.shuffle(&mut self.rng);
                        self.rng_queue.pop().unwrap() // guaranteed to exist
                    }
                }
            }
        };
        // self.next_shape_idx = self.rng.gen_range(0, TETRIMINOES.len());
        self.rotation = 0;
        self.base = if self.curr_shape_idx == 0 {
            // if it is horizontal I shape
            Point(0, self.field.width as i32 / 2)
        } else {
            Point(1, self.field.width as i32 / 2)
        };
        if let Some(cells) = self.try_current_shape(&self.base, self.rotation) {
            for i in 0..cells.len() {
                self.curr_cells[i] = cells[i];
            }
        } else {
            // restore index to avoid incorrect color change of the last tetrimino,
            // that caused the game over
            self.next_shape_idx = self.curr_shape_idx;
            self.curr_shape_idx = prev_shape_idx;
            self.game_over = true;
        }
    }

    pub fn drop_current_shape(&mut self) -> (i32, Option<Vec<Point>>) {
        let mut i = self.base.0;
        // the last valid position on the path
        let mut cur_cells: Option<Vec<Point>> = None;
        loop {
            let base_new = Point(i, self.base.1);
            let cells = self.try_current_shape(&base_new, self.rotation);
            if cells.is_none() {
                break;
            }
            cur_cells = cells;
            i += 1;
        }
        (i, cur_cells)
    }

    pub fn step(&mut self, action: Action) -> (usize, bool) {
        if self.game_over {
            return (0, true);
        }
        let mut lines_burnt = 0;
        match action {
            Action::Tick => {
                // clear current
                let base_new = Point(self.base.0 + 1, self.base.1);
                if let Some(cells) = self.try_current_shape(&base_new, self.rotation) {
                    self.base = base_new;
                    for i in 0..cells.len() {
                        self.curr_cells[i] = cells[i];
                    }
                } else {
                    self.draw_current_shape();
                    lines_burnt = self.burn_lines();
                    self.spawn_next_shape();
                }
            }
            Action::HardDrop => {
                let (ish, cur_cells) = self.drop_current_shape();
                if let Some(cells) = cur_cells {
                    for i in 0..cells.len() {
                        self.curr_cells[i] = cells[i];
                    }
                    self.base = Point(ish, self.base.1);
                    self.draw_current_shape();
                    lines_burnt = self.burn_lines();
                    self.spawn_next_shape();
                }
            }
            Action::Down => {
                // clear current
                let base_new = Point(self.base.0 + 1, self.base.1);
                if let Some(cells) = self.try_current_shape(&base_new, self.rotation) {
                    self.base = base_new;
                    for i in 0..cells.len() {
                        self.curr_cells[i] = cells[i];
                    }
                }
            }
            Action::Left => {
                // clear current
                let base_new = Point(self.base.0, self.base.1 - 1);
                if let Some(cells) = self.try_current_shape(&base_new, self.rotation) {
                    self.base = base_new;
                    for i in 0..cells.len() {
                        self.curr_cells[i] = cells[i];
                    }
                }
            }
            Action::Right => {
                // clear current
                let base_new = Point(self.base.0, self.base.1 + 1);
                if let Some(cells) = self.try_current_shape(&base_new, self.rotation) {
                    self.base = base_new;
                    for i in 0..cells.len() {
                        self.curr_cells[i] = cells[i];
                    }
                }
            }
            Action::RotateCCW => {
                // clear current
                let rotation_new = (self.rotation + 3) % 4;
                let transition = (self.rotation, rotation_new);
                if let Some((base, cells)) = self.wall_kick_current_shape(transition) {
                    self.base = base;
                    self.rotation = rotation_new;
                    for i in 0..cells.len() {
                        self.curr_cells[i] = cells[i];
                    }
                }
            }
            Action::RotateCW => {
                // clear current
                let rotation_new = (self.rotation + 1) % 4;
                let transition = (self.rotation, rotation_new);
                if let Some((base, cells)) = self.wall_kick_current_shape(transition) {
                    self.base = base;
                    self.rotation = rotation_new;
                    for i in 0..cells.len() {
                        self.curr_cells[i] = cells[i];
                    }
                }
            }
        }
        return (lines_burnt, false);
    }

/*
*/


    /// In wide mode the result is similar to (without comment sign)
    /// ```
    /// // .    .    .    .    .
    /// //
    /// // .    .    .    .    .
    /// //       %%%%
    /// // .    .%%%%.    .    .
    /// //       %%%%
    /// // .    .%%%%.    .    .
    /// //       %%%% %%%%
    /// // .    .%%%%.%%%%.    .
    /// //
    /// // .    .    .    .    .
    /// ```
    pub fn prettify_game_state(&self, rewind: bool, _use_colors: bool, wide: bool) -> String {
        let fill_block = "\u{25AE}".repeat(4);
        let m = self.field.height;
        let n = self.field.width;
        let mut result = String::with_capacity(m * (10 * n + 1) + 2);
        result.push_str("\r\n");
        result.push_str(&format!("score: {}\r\n", self.score));
        result.push_str(&format!("next shape: {}\r\n", &TETRIMINOES[self.next_shape_idx]
            .style.apply_to(&self.next_shape_idx.to_string())));
        result.push_str(&format!("current shape: {}\r\n", &TETRIMINOES[self.curr_shape_idx]
            .style.apply_to(&self.curr_shape_idx.to_string())));

        // now put all the stuff
        if wide {
            // ----------------
            // wide mode render
            for i in 0..m {
                if i == 0 {
                    for j in 0..n {
                        if j == 0 { result.push('.'); }
                        result.push_str("   .");
                    }
                    result.push_str("\r\n");
                }
                for j in 0..n {
                    if j == 0 {
                        result.push(' ');
                    }
                    let ij = Point(i as i32, j as i32);
                    let cell = if self.curr_cells.iter().all(|c| *c != ij) {
                        self.field.cells[i][j]
                    } else {
                        self.curr_shape_idx as u8 + 1
                    };
                    if cell == 0 {
                        result.push_str("    ");
                    } else {
                        let color: Style = TETRIMINOES[cell as usize - 1].style;
                        result.push_str(&color.apply_to(&fill_block));
                    }
                }
                result.push_str("\r\n");
                for j in 0..n {
                    if j == 0 {
                        result.push('.');
                    }
                    let ij = Point(i as i32, j as i32);
                    let cell = if self.curr_cells.iter().all(|c| *c != ij) {
                        self.field.cells[i][j]
                    } else {
                        self.curr_shape_idx as u8 + 1
                    };
                    if cell == 0 {
                        result.push_str("   .");
                    } else {
                        let color: Style = TETRIMINOES[cell as usize - 1].style;
                        result.push_str(&color.apply_to(&fill_block));
                    }
                }
                result.push_str("\r\n");
            }
            if rewind {
                for _ in 0..(2 * m + 6) {
                    result.push_str("\x1B[A") // up
                }
            }
        } else {
            // -------------
            // compact mode render
            let empty_style = Style::Empty;
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
                    let ij = Point(i as i32, j as i32);
                    let cell = if self.curr_cells.iter().all(|c| *c != ij) {
                        self.field.cells[i][j]
                    } else {
                        self.curr_shape_idx as u8 + 1
                    };
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
                result.push_str("\r\n");
            }
            if rewind {
                for _ in 0..(m + 5) {
                    result.push_str("\x1B[A") // up
                }
            }
        }
        result
    }
}

pub fn try_shape(field: &Field, base: &Point, rotation: i8, shape: &Tetrimino) -> Option<Vec<Point>> {
    let mut points = rotate(&shape, rotation);
    for p in &points {
        let i = base.0 + p.0;
        let j = base.1 + p.1;
        if i < 0 || field.height as i32 <= i {
            return None;
        } else if j < 0 || field.width as i32 <= j {
            return None;
        } else if field.cells[i as usize][j as usize] != 0 {
            return None;
        }
    }
    // shift points w.r.t. base
    for p in &mut points {
        p.0 = base.0 + p.0;
        p.1 = base.1 + p.1;
    }
    Some(points)
}

pub fn rotate(shape: &Tetrimino, r: i8) -> Vec<Point> {
    // modulo, NOT the remainder, see https://stackoverflow.com/a/41422009/5066426
    let r = (r % 4 + 4) % 4;
    let mut p = shape.clone();
    for _ in 0..r {
        for d in &mut p.diffs {
            // rotate clockwise (-2, 1) => (1, 2),
            // i.e. negate the first coordinate and then swap
            let t = -d.0;
            d.0 = d.1;
            d.1 = t;
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

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str(&self.prettify_game_state(false, false, false))?;
        Ok(())
    }
}
