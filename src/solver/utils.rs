use crate::location::Location;
use crate::grid::Grid;
use crate::config::CELL_COUNT;
use crate::cell::Cell;


/// Calculate and initialize the wave function of all blank cells in the given grid.
/// The input grid is expected to have no wave functions.
pub fn initialize_waves(grid: &mut Grid) {

    for i in 0..CELL_COUNT {
        
        // Assume there aren't Cell::Uncertain variants on the board
        // This function should be called on a fully unsolved board with no wave function marks (only certain and blank cells)
        if matches!(grid.get_index(i), Cell::Blank) {

            let location = Location::from_index(i);

            let wave = grid.wave_at(location);

            if let Some(collapsed) = wave.collapsed() {

                grid.update_collapse(location, collapsed).expect("Should not fail because the board is solvable");

            } else {
                grid.set_index(i, Cell::Uncertain { wave });
            }
        }
    }
}

