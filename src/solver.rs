use std::collections::HashSet;

use crate::config::CELL_COUNT;
use crate::location::Location;
use crate::grid::Grid;
use crate::cell::Cell;


pub fn solve_bruteforce(grid: &Grid) -> Grid {

    let mut new_grid = grid.clone();

    let mut total_collapsed = 0;

    /*
        1. Calculate the wave function of all blank cells.
        2. Choose the cell with the lowest entropy, assuming it hasn't collapsed yet
        3. Save the board state and create a new branch.
        4. Choose one of the possible states and collapse the cell.
        5. Continue solving the new branch:
            - if the branched board is solvable, return the solved board.
            - if the branched board is not solvable, revert back to the previous branch and continue solving from there.
    */

    // Calculate the wave function of all blank cells 
    for i in 0..CELL_COUNT {
        
        // Assume there aren't Cell::Uncertain variants on the board
        // This function should be called on a fully unsolved board with no wave function marks (only certain and blank cells)
        if matches!(new_grid.get_index(i), Cell::Blank) {

            let location = Location::from_index(i);

            let wave = new_grid.wave_at(location);

            if let Some(collapsed) = wave.collapsed() {

                new_grid.update_collapse(location, collapsed, &mut total_collapsed).expect("Should not fail because the board is solvable");

            } else {
                new_grid.set_index(i, Cell::Uncertain { wave });
            }
        }
    }

    solve_bruteforce_internal(new_grid, 0, total_collapsed).expect("The board should be solvable")
}


fn solve_bruteforce_internal(grid: Grid, depth: usize, total_collapsed: usize) -> Result<Grid, ()> {

    println!("Backtracking depth: {depth}, total collapsed: {total_collapsed}");
    // println!("{grid}");

    let mut visited: HashSet<Location> = HashSet::new();

    while visited.len() < CELL_COUNT {

        // Choose the cell with the lowest entropy, assuming it hasn't collapsed yet

        let (location, mut wave) = match grid.lowest_entropy_except(&visited) {

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
        while let Some(collapsed) = wave.collapse_first() {

            // Mark this digit as already used
            wave.remove_possibility(collapsed);

            let mut branch = grid.clone();

            let mut total_collapsed = total_collapsed;

            if branch.update_collapse(location, collapsed, &mut total_collapsed).is_err() {
                // Assuming the board is solvable, collapsing this cell with this digit leads to an invalid board.
                // Proceed the solving process by trying with the next possible state.
                continue;
            }

            // println!("Trying board:\n{branch}");

            // The update was successful, proceed the solving process
            if let Ok(solved_grid) = solve_bruteforce_internal(branch, depth + 1, total_collapsed) {
                // If a solution is found, stop solving and unwind
                return Ok(solved_grid);
            }

        }

    }

    // After visiting all available cells, the board could not be solved on this branch
    Err(())
}


#[allow(dead_code, unused_variables)]
pub fn solve_analyze_neighboring_waves(grid: &Grid) -> Grid {
    todo!()
}

