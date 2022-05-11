use crate::solver::{self, solver};

pub type Clue = u8;
#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Unknown,
    Filled,
    Blocked,
}

pub enum RowClue {
    Single(Clue),
    Group(Vec<Clue>),
}

impl RowClue {
    pub fn parse(input: &str) -> Option<Vec<RowClue>> {
        let mut clues = Vec::new();
        for clue in input.split('.') {
            let mut numbers = clue.split(',');
            let first: u8 = numbers.next()?.parse().ok()?;
            clues.push(match numbers.next() {
                Some(v) => {
                    let v = v.parse().ok()?;
                    let mut out = vec![first, v];
                    for v in numbers {
                        out.push(v.parse().ok()?);
                    }
                    RowClue::Group(out)
                }
                None => RowClue::Single(first),
            });
        }
        Some(clues)
    }
}

pub struct Board {
    clue_row: Vec<RowClue>,
    clue_col: Vec<RowClue>,
    board: Vec<Vec<Cell>>,
}

impl Board {
    pub fn parse(clue_row: &str, clue_col: &str) -> Option<Board> {
        let clue_row = RowClue::parse(clue_row)?;
        let clue_col = RowClue::parse(clue_col)?;
        let width = clue_row.len();
        let height = clue_col.len();
        Some(Board {
            clue_col,
            clue_row,
            board: vec![vec![Cell::Unknown; width]; height],
        })
    }

    pub fn low_hanging(&mut self) {
        // X
        for (row, clue) in self.board.iter_mut().zip(self.clue_row.iter()) {
            solver::low_hanging(row, clue);
        }

        // Y
        let mut row = Vec::with_capacity(self.board.len());
        for x in 0..self.board[0].len() {
            // Add the elements
            for y in 0..self.board.len() {
                row.push(self.board[y][x]);
            }
            solver::low_hanging(&mut row, &self.clue_col[x]);
            // Write them back
            for (y, cell) in row.drain(..).enumerate() {
                self.board[y][x] = cell;
            }
            row.clear();
        }
    }

    pub fn solve_step(&mut self) {
        // Rows
        for (row, clue) in self.board.iter_mut().zip(self.clue_row.iter()) {
            solver(row, clue);
        }
        // Columns
        let columns = self.board[0].len();
        let rows = self.board.len();
        let mut col = Vec::with_capacity(columns);
        for (x, clue) in self.clue_col.iter().enumerate() {
            for y in 0..rows {
                col.push(self.board[y][x]);
            }
            solver(&mut col, clue);
            // Then we write this back into the board
            for (y, val) in col.drain(..).enumerate() {
                self.board[y][x] = val;
            }
        }
    }

    pub fn solved_percentage(&self) -> f32 {
        let unknown = self
            .board
            .iter()
            .flat_map(|row| row.iter())
            .filter(|cell| matches!(cell, Cell::Unknown))
            .count();
        let total_cells = self.board.len() * self.board[0].len();
        (unknown as f32) / (total_cells as f32)
    }

    pub fn export(&self) -> Option<Vec<Vec<bool>>> {
        let mut rows = Vec::with_capacity(self.board.len());
        for row in &self.board {
            let mut export_row = Vec::with_capacity(row.len());
            for cell in row {
                export_row.push(match cell {
                    Cell::Filled => true,
                    Cell::Blocked => false,
                    Cell::Unknown => return None,
                });
            }
            rows.push(export_row);
        }
        Some(rows)
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn export_js(&self) -> Option<js_sys::Array> {
        let rows = js_sys::Array::new_with_length(self.board.len() as u32);
        for (y, row) in self.board.iter().enumerate() {
            let export_row = js_sys::Array::new_with_length(row.len() as u32);
            for (x, cell) in row.iter().enumerate() {
                export_row.set(
                    x as u32,
                    match cell {
                        Cell::Filled => true,
                        Cell::Blocked => false,
                        Cell::Unknown => return None,
                    }
                    .into(),
                );
            }
            rows.set(y as u32, export_row.into());
        }
        Some(rows)
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Unknown => write!(f, "?"),
            Cell::Filled => write!(f, "*"),
            Cell::Blocked => write!(f, " "),
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.board {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
