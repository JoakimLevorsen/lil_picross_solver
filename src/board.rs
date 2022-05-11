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
    pub fn parse(input: &str) -> Vec<RowClue> {
        input
            .split('.')
            .map(|s| {
                let mut numbers = s.split(',').map(|number| number.parse().unwrap());
                let first = numbers.next().unwrap();
                match numbers.next() {
                    Some(v) => {
                        let mut out = vec![first, v];
                        for v in numbers {
                            out.push(v)
                        }
                        RowClue::Group(out)
                    }
                    None => RowClue::Single(first),
                }
            })
            .collect()
    }
}

pub struct Board {
    clue_row: Vec<RowClue>,
    clue_col: Vec<RowClue>,
    board: Vec<Vec<Cell>>,
}

impl Board {
    pub fn parse(clue_row: &str, clue_col: &str) -> Option<Board> {
        let clue_row = RowClue::parse(clue_row);
        let clue_col = RowClue::parse(clue_col);
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
                row.push(self.board[y][x])
            }
            solver::low_hanging(&mut row, &self.clue_col[x]);
            // Write them back
            for y in 0..self.board.len() {
                self.board[y][x] = row[y]
            }
            row.clear()
        }
    }

    pub fn solve_step(&mut self) {
        // Rows
        for (row, clue) in self.board.iter_mut().zip(self.clue_row.iter()) {
            solver(row, clue)
        }
        // Columns
        let columns = self.board[0].len();
        let rows = self.board.len();
        let mut col = Vec::with_capacity(columns);
        for (x, clue) in self.clue_col.iter().enumerate() {
            for y in 0..rows {
                col.push(self.board[y][x])
            }
            solver(&mut col, clue);
            // Then we write this back into the board
            for (y, val) in col.drain(..).enumerate() {
                self.board[y][x] = val;
            }
        }
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
