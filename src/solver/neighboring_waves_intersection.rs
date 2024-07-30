use crate::grid::Grid;

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
pub fn solve(grid: &Grid) -> Grid {
    
    let mut new_grid = grid.clone();

    utils::initialize_waves(&mut new_grid);

    // for cell in new_grid.get_row(location)

    new_grid
}

