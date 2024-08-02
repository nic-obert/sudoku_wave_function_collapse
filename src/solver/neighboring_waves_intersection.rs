use crate::grid::{self, Cell, Grid, Location};
use crate::config::DIGIT_BASE;

use super::utils;


#[allow(dead_code)]
/**
    1. Calculate the wave function of all blank cells.
    2. Iterate through all rows, columns, and boxes:
        3. Search for a unique digit in each row, column, or box:
            - if no unique digit is found, continue searching.
            - if a unique digit is found in a cell, the cell collapses into that digit,
                recursively update the sector and continue searching.
    4. If no unique digit is found in this pass, do a bruteforce backtracking step and repeat.
 */
pub fn solve(base_grid: &Grid) -> Grid {
    
    let mut grid = base_grid.clone();

    utils::initialize_waves(&mut grid);

    if grid.is_solved() {
        grid
    } else {
        solve_backtracking(grid).expect("Board should be solvable")
    }
}


fn solve_backtracking(mut grid: Grid) -> Result<Grid, ()> {

    // Try to solve the board by analyzing the neighboring wave functions.
    while pass_wave_group(&mut grid, grid::iter_rows())?
        || pass_wave_group(&mut grid, grid::iter_columns())?
        || pass_wave_group(&mut grid, grid::iter_boxes())?
    { }

    // If nothing changed from the previous iteration, the board can no longer be updated using this technique.

    if let Some((location, wave)) = grid.lowest_entropy() {

        // Try collapsing each state to see which produces a correct board
        for state in wave.states() {

            let mut branch = grid.clone();

            if branch.update_collapse(location, state).is_err() {
                continue;
            }
            
            if let Ok(solved_board) = solve_backtracking(branch) {
                return Ok(solved_board);
            }
        }
        
        // No valid state was found for this cell in this branch. Backtrack
        Err(())
    } else {
        // Grid is solved
        Ok(grid)
    }
}


/// Do a pass over the given iterator, collapsing wave functions when possible.
/// Return whether any wave function was collapsed.
/// This function fails if the board reaches an unsolvable state.
fn pass_wave_group(grid: &mut Grid, it: impl Iterator<Item = impl Iterator<Item = Location>>) -> Result<bool, ()> {

    let mut collapsed_any = false;

    #[derive(Clone, Copy)]
    enum State {
        Unique (Location),
        NonUniqueOrAlreadyUsed,
        Unseen,
    }

    for iter in it {

        let mut states = [State::Unseen; DIGIT_BASE];

        for cell in iter {

            match grid.get_at(cell) {

                Cell::Certain { digit } => {
                    match states[digit as usize - 1] {
                        State::Unique(_) |
                        State::Unseen 
                            => states[digit as usize - 1] = State::NonUniqueOrAlreadyUsed,
                        State::NonUniqueOrAlreadyUsed => ()
                    }
                },

                Cell::Uncertain { wave } => {
                    for state in wave.states() {
                        match states[state as usize - 1] {
                            State::Unique(_) => states[state as usize - 1] = State::NonUniqueOrAlreadyUsed,
                            State::Unseen => states[state as usize - 1] = State::Unique(cell),
                            State::NonUniqueOrAlreadyUsed => ()
                        }
                    }
                },

                Cell::Blank => unreachable!("The board is initialized beforehand"),
            }

        }

        for (i, &state) in states.iter().enumerate() {
            // If a state is unique within its row, column, or box, it's guaranteed to be the correct state
            if let State::Unique(location) = state {
                grid.update_collapse(location, i as u8 + 1)?;
                collapsed_any = true;
            }
        }

    }

    Ok(collapsed_any)
}


#[cfg(test)]
mod tests {

    // use crate::file_utils::save_board;
    use crate::parse_cell;
    use crate::create_board;

    use super::*;

    #[test]
    fn solve_easy_solvable() {

        let board = create_board!([
            (),3,1,(),(),(),(),(),(),
            6,4,(),2,(),(),3,8,(),
            (),2,(),6,(),1,(),5,9,
            4,9,6,(),2,3,5,(),(),
            2,(),8,(),5,4,(),(),(),
            (),5,3,(),6,(),8,2,4,
            (),6,(),3,(),(),1,(),(),
            (),(),5,(),9,(),(),7,3,
            (),(),(),(),1,(),2,(),8
        ]);

        // save_board(&board, "test_boards/easy.json");

        solve(&board);

        assert!(board.check_valid());

    }


    #[test]
    fn solve_medium_solvable() {

        let board = create_board!([
            (),(),(),8,(),(),(),3,(),
            (),1,4,(),(),5,7,(),(),
            3,(),9,(),(),(),5,6,(),
            (),3,6,7,8,(),(),(),(),
            1,(),(),(),(),(),4,(),(),
            (),(),(),(),1,3,(),7,(),
            (),(),(),6,(),(),(),4,9,
            6,4,2,(),3,9,(),5,7,
            7,(),3,4,5,8,(),2,1
        ]);

        // save_board(&board, "test_boards/medium.json");

        solve(&board);

        assert!(board.check_valid());

    }

    
    #[test]
    fn solve_hard_solvable() {

        let board = create_board!([
            (),(),(),(),(),(),(),(),(),
            6,8,(),9,(),5,4,(),(),
            (),(),(),(),(),3,(),7,1,
            4,7,(),(),(),(),8,(),6,
            1,(),2,(),(),8,(),(),(),
            (),(),(),(),6,4,1,(),2,
            7,(),6,3,4,(),(),1,(),
            3,(),5,8,(),(),(),(),(),
            (),9,(),2,(),(),7,(),()
        ]);

        // save_board(&board, "test_boards/hard.json");

        solve(&board);

        assert!(board.check_valid());
    }

}