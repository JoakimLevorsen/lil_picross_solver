use crate::board::{Cell, Clue, RowClue};

pub fn low_hanging(row: &mut Vec<Cell>, clue: &RowClue) {
    match clue {
        RowClue::Single(v) if *v == 0 => {
            for cell in row {
                *cell = Cell::Blocked
            }
        }
        RowClue::Single(v) if *v as usize == row.len() => {
            for cell in row {
                *cell = Cell::Filled
            }
        }
        RowClue::Group(clues)
            if (clues.iter().copied().sum::<u8>() as usize) + clues.len() - 1 == row.len() =>
        {
            // This means we can just enter the pattern
            let zippable = clues.iter().copied().flat_map(|max| {
                (0..max)
                    .map(|_| Cell::Filled)
                    .chain(std::iter::once(Cell::Blocked))
            });

            for (cell, value) in row.iter_mut().zip(zippable) {
                *cell = value
            }
        }
        _ => (),
    }
}

pub fn solver(row: &mut Vec<Cell>, clue: &RowClue) {
    match clue {
        RowClue::Single(clue) => single_row_solver(row, *clue),
        RowClue::Group(group) => multi_row_solver(row, group),
    }
}

fn single_row_solver(row: &mut Vec<Cell>, clue: Clue) {
    // if clue == 0 {
    //     for cell in row {
    //         *cell = Some(false);
    //     }
    // }
    // if clue as usize == row.len() {
    //     for cell in row {
    //         *cell = Some(true);
    //     }
    // }
    // // We only have one clue so is any cell filled and can act as anchors?
    // let min_anchor = None;
    // let max_anchor = None;
    // let mut iter = row.iter().enumerate().peekable();
    // // We skip until
    // iter = iter.skip_while(predicate);
    let mut possible = Vec::new();
    let spacing = row.len() - clue as usize;
    'option_looker: for i in 0..spacing {
        let mut row_iter = row.iter();
        // We need to ensure i initial empty, then clue filled, then spacing - i - len empty
        // Any conflicts mean an abort
        for _ in 0..i {
            match row_iter.next() {
                Some(Cell::Blocked | Cell::Unknown) => (),
                _ => continue 'option_looker,
            }
        }
        for _ in 0..clue {
            match row_iter.next() {
                Some(Cell::Filled | Cell::Unknown) => (),
                _ => continue 'option_looker,
            }
        }
        for _ in 0..(row.len() - (clue as usize) - i) {
            match row_iter.next() {
                Some(Cell::Blocked | Cell::Unknown) => (),
                _ => continue 'option_looker,
            }
        }
        // If we get here this combination is valid
        let mut examined = Vec::with_capacity(row.len());
        for _ in 0..i {
            examined.push(Cell::Blocked);
        }
        for _ in 0..clue {
            examined.push(Cell::Filled);
        }
        for _ in 0..(row.len() - (clue as usize) - i) {
            examined.push(Cell::Blocked);
        }
        possible.push(examined);
    }
    // Then we can combine all the possible to a single option
    if possible.len() == 1 {
        *row = possible.pop().unwrap();
        return;
    }
    let mut combined = match possible.pop() {
        Some(v) => v,
        None => return,
    };
    for possible in possible {
        for (combined, add) in combined.iter_mut().zip(possible.into_iter()) {
            *combined = match (*combined, add) {
                (Cell::Blocked, Cell::Filled) | (Cell::Filled, Cell::Blocked) => Cell::Unknown,
                (Cell::Unknown, _) | (_, Cell::Unknown) => Cell::Unknown,
                (Cell::Filled, Cell::Filled) => Cell::Filled,
                (Cell::Blocked, Cell::Blocked) => Cell::Blocked,
            }
        }
    }
    // We then apply the combined to the input
    for (current, combined) in row.iter_mut().zip(combined.into_iter()) {
        *current = match (*current, combined) {
            (Cell::Unknown, v) | (v, Cell::Unknown) => v,
            (Cell::Blocked, Cell::Filled) | (Cell::Filled, Cell::Blocked) => {
                panic!("Tried to fuck something up here")
            }
            v @ (Cell::Filled, Cell::Filled) | v @ (Cell::Blocked, Cell::Blocked) => v.0,
        }
    }
}

fn multi_row_solver(row: &mut Vec<Cell>, clue: &[Clue]) {
    let mut options = Vec::new();
    recursive_option_finder(
        clue,
        row.len(),
        &Vec::with_capacity(row.len()),
        &*row,
        &mut options,
    );
    let mut combined = match options.pop() {
        Some(v) => v,
        None => return,
    };
    // If only one option we just write that
    if options.is_empty() {
        *row = combined;
        return;
    }
    for possible in options {
        for (combined, add) in combined.iter_mut().zip(possible.into_iter()) {
            *combined = match (*combined, add) {
                (Cell::Blocked, Cell::Filled) | (Cell::Filled, Cell::Blocked) => Cell::Unknown,
                (Cell::Unknown, _) | (_, Cell::Unknown) => Cell::Unknown,
                (Cell::Filled, Cell::Filled) => Cell::Filled,
                (Cell::Blocked, Cell::Blocked) => Cell::Blocked,
            }
        }
    }
    // We then apply the combined to the input
    for (current, combined) in row.iter_mut().zip(combined.into_iter()) {
        *current = match (*current, combined) {
            (Cell::Unknown, v) | (v, Cell::Unknown) => v,
            (Cell::Blocked, Cell::Filled) | (Cell::Filled, Cell::Blocked) => {
                panic!("Tried to fuck something up here")
            }
            v @ (Cell::Filled, Cell::Filled) | v @ (Cell::Blocked, Cell::Blocked) => v.0,
        }
    }
}

fn recursive_option_finder(
    clues: &[Clue],
    max_len: usize,
    previous: &[Cell],
    current: &[Cell],
    options: &mut Vec<Vec<Cell>>,
) {
    if max_len == previous.len() {
        return;
    }
    let clue = match clues.get(0) {
        Some(v) => *v,
        None => return,
    };
    let clues = &clues[1..];
    let len_remaining = max_len - previous.len() - (clue as usize);
    // We must also mark atleast 1 space between all remaining clues
    let len_remaining = if clues.is_empty() {
        len_remaining
    } else {
        len_remaining - (clues.len() - 1)
    };
    // The first clue does not require any spacing
    let min = if previous.is_empty() { 0 } else { 1 };
    // If we need 1 space, and the minimum is 1, we do exactly 1 run with 1 len
    // let range = if len_remaining == 1 && min == 1 {
    //     1..2
    // } else {
    //     min..len_remaining
    // };
    'len_examiner: for i in min..=len_remaining {
        let mut row = previous.to_vec();
        let relevant_current = &current[row.len()..];
        let mut current_iter = relevant_current.iter();
        // We add our spacing and check its legal
        for _ in 0..i {
            match current_iter.next() {
                Some(Cell::Blocked | Cell::Unknown) => (),
                _ => continue 'len_examiner,
            }
            row.push(Cell::Blocked);
        }
        // We add the cells
        for _ in 0..clue {
            match current_iter.next() {
                Some(Cell::Filled | Cell::Unknown) => (),
                _ => continue 'len_examiner,
            }
            row.push(Cell::Filled);
        }
        // Are we out of clues? If so this row should be appended end spacing and add to options
        if clues.is_empty() {
            for _ in 0..(max_len - row.len()) {
                match current_iter.next() {
                    Some(Cell::Blocked | Cell::Unknown) => (),
                    _ => continue 'len_examiner,
                }
                row.push(Cell::Blocked);
            }
            options.push(row);
        } else {
            // We continue recursively
            recursive_option_finder(clues, max_len, &row, current, options)
        }
    }
}
