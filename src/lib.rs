#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::cast_precision_loss)]

use wasm_bindgen::prelude::*;

pub use board::Board;

mod board;
mod solver;

#[wasm_bindgen]
pub fn solve_js(clues_row: &str, clues_col: &str) -> Option<js_sys::Array> {
    solve(clues_row, clues_col)?.0.export_js()
}

pub fn solve(clues_row: &str, clues_col: &str) -> Option<(Board, u32)> {
    const ERROR_MARGIN: f32 = 0.001;

    let mut board = Board::parse(clues_row, clues_col)?;
    board.low_hanging();
    let mut solved_percentage = board.solved_percentage();
    let mut steps = 0;
    loop {
        board.solve_step();
        steps += 1;
        let now = board.solved_percentage();
        if (solved_percentage - now).abs() < ERROR_MARGIN {
            break;
        }
        solved_percentage = now;
    }
    Some((board, steps))
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
