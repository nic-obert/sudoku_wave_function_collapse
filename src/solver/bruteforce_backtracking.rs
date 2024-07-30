use std::collections::HashSet;

use crate::config::CELL_COUNT;
use crate::location::Location;
use crate::grid::Grid;

use super::utils;


/**
    1. Calculate the wave function of all blank cells.
    2. Choose the cell with the lowest entropy, assuming it hasn't collapsed yet
    3. Save the board state and create a new branch.
    4. Choose one of the possible states and collapse the cell.
    5. Continue solving the new branch:
        - if the branched board is solvable, return the solved board.
        - if the branched board is not solvable, revert back to the previous branch and continue solving from there.
*/
pub fn solve(grid: &Grid) -> Grid {

    let mut new_grid = grid.clone();

    utils::initialize_waves(&mut new_grid);

    solve_bruteforce_internal(new_grid).expect("The board should be solvable")
}


fn solve_bruteforce_internal(grid: Grid) -> Result<Grid, ()> {

    // println!("{grid}");

    let mut visited: HashSet<Location> = HashSet::new();

    while visited.len() < CELL_COUNT {

        // Choose the cell with the lowest entropy, assuming it hasn't collapsed yet

        let (location, wave) = match grid.lowest_entropy_except(&visited) {

            // No uncertain cells found, stop solving
            Ok(None) => return Ok(grid),

            // All uncertain cells were already visited, this branch isn't solvable
            Err(()) => return Err(()),

            // This branch may be solvable
            Ok(Some(x)) => x
        };

        // Mark this cell as visited so that the next iteration can try to collapse another cell
        visited.insert(location);

        // Try every possible state of the wave function until the board is solved
        for collapsed in wave.states() {

            let mut branch = grid.clone();

            if branch.update_collapse(location, collapsed).is_err() {
                // Assuming the board is solvable, collapsing this cell with this digit leads to an invalid board.
                // Proceed the solving process by trying with the next possible state.
                continue;
            }

            // println!("Trying board:\n{branch}");

            // The update was successful, proceed the solving process
            if let Ok(solved_grid) = solve_bruteforce_internal(branch) {
                // If a solution is found, stop solving and unwind
                return Ok(solved_grid);
            }

        }

    }

    // After visiting all available cells, the board could not be solved on this branch
    Err(())
}

