use std::collections::HashSet;

use rand::seq::SliceRandom;

use crate::config::CELL_COUNT;
use crate::solver::neighboring_waves_intersection::has_unique_solution;

use super::{Grid, Location, Cell, Digit, Entropy, WaveFunction};
use super::grid_iter;


impl Grid {

    /// Collapse the specified cell and update all the cells in its sector accordingly. 
    /// Collapse all cells that reach a collapsible state as a consequence of a previous collapse.
    /// This function fails if the sudoku rules are not satisfied after the collapse.
    pub fn update_collapse(&mut self, location: Location, collapsed_digit: Digit) -> Result<(), ()> {

        let mut to_collapse = vec![(location, collapsed_digit)];

        while let Some((location, collapsed_digit)) = to_collapse.pop() {

            if matches!(self.get_at(location), Cell::Certain { digit } if digit == collapsed_digit) {
                // Already updated by a previous iteration
                continue;
            }

            if matches!(self.get_at(location), Cell::Certain { .. }) {
                return Err(());
            }

            self.set_at(location, Cell::Certain { digit: collapsed_digit });
    
            for cell in grid_iter::iter_sector(location) {
    
                match self.get_at(cell) {
                    
                    Cell::Certain { digit } => {
                        if cell != location && digit == collapsed_digit {
                            return Err(());
                        }
                    },
                    
                    Cell::Uncertain { mut wave } => {
                        
                        wave.remove_possibility(collapsed_digit);
                        
                        if let Some(newly_collapsed) = wave.collapsed() {
                            to_collapse.push((cell, newly_collapsed));
                        } else {
                            self.set_at(cell, Cell::Uncertain { wave });
                        };
                    },
    
                    Cell::Blank => {
                        // Do nothing. Blank cells will be updated directly by the solver.
                    }
                }
    
            }

        }

        Ok(())
    }


    pub fn wave_at(&self, location: Location) -> WaveFunction {

        let mut wave = WaveFunction::new_max_entropy();

        for cell in grid_iter::iter_sector(location) {

            // Ignore the compatibility of single wave functions in the sector 

            if let Cell::Certain { digit } = self.get_at(cell) {
                wave.remove_possibility(digit);
            }

        }

        wave
    }


    pub fn is_solved(&self) -> bool {

        for cell in self.cells.iter() {
            if !matches!(cell, Cell::Certain { .. }) {
                return false;
            }
        }

        true
    }


    pub fn new_random() -> Self {

        let rng = rand::thread_rng();

        'gen_attempt: loop {

            let mut grid = Self::new_max_entropy();

            for i in 0..CELL_COUNT {

                match grid.get_index(i) {

                    Cell::Uncertain { wave } => {

                        let collapsed = wave.collapse_random(rng.clone()).expect("Should be valid because of the wave function");
                        
                        if grid.update_collapse(Location::from_index(i), collapsed).is_err() {
                            continue 'gen_attempt;
                        }

                    },
                    
                    Cell::Certain { .. } => {
                        // Do nothing, the wave function is already collapsed
                    },

                    Cell::Blank => unreachable!()
                }

            }

            return grid;
        }
    }


    #[allow(dead_code)]
    pub fn check_valid(&self) -> bool {

        for i in 0..CELL_COUNT {

            match self.get_index(i) {

                Cell::Certain { digit } => {

                    let location = Location::from_index(i);

                    for neighbor in grid_iter::iter_sector(location) {

                        match self.get_at(neighbor) {

                            Cell::Certain { digit: neigh_digit }
                                if neigh_digit == digit && location != neighbor
                                    => return false,

                            _ => { }
                        }
                    }
                },

                Cell::Uncertain { .. } |
                Cell::Blank
                 => {
                    // Do nothing, this cell is still uncertain.
                    // We may check if the wave function is compatible with the waves of its sector, but that's a lot of work
                },
            }

        }

        true
    }


    pub fn with_random_blank_cells(&self, blank_cell_cap: u8) -> Self {

        let mut new_grid = self.clone();

        let mut rng = rand::thread_rng();

        let mut cells: Vec<usize> = (0..CELL_COUNT).collect();
        cells.shuffle(&mut rng);

        for (cells_cleared, &i) in cells.iter().enumerate() {

            if blank_cell_cap == cells_cleared as u8 {
                break;
            }

            let old_cell_state = new_grid.get_index(i);
            
            new_grid.set_index(i, Cell::Blank);

            if !has_unique_solution(&new_grid) {
                new_grid.set_index(i, old_cell_state);
            }

        }

        new_grid
    }


    /// Return the cell with the lowest entropy among the uncertain cells.
    /// Note that the lowest possible valid entropy is 2 beacuse an entropy value of 1 would collapse the wave function.
    /// This function returns None if all cells are certain (board is solved)
    pub fn lowest_entropy(&self) -> Option<(Location, WaveFunction)> {

        let mut lowest: Option<(Location, WaveFunction)> = None;
        let mut lowest_entropy = Entropy::MAX;

        for i in 0..CELL_COUNT {

            if let Cell::Uncertain { wave } = self.get_index(i) {

                let local_entropy = wave.entropy();

                let location = Location::from_index(i);

                if local_entropy == 2 {
                    return Some((location, wave));
                }

                if local_entropy < lowest_entropy {
                    lowest = Some((location, wave));
                    lowest_entropy = local_entropy;
                }
            }
        }

        lowest
    }


    /// Return the cell with the lowest entropy among the uncertain cells, skipping the cells that were already visited.
    /// Note that the lowest possible valid entropy is 2 beacuse an entropy value of 1 would collapse the wave function.
    /// This function fails if all the uncertain cells were already visited.
    pub fn lowest_entropy_except(&self, visited_cells: &HashSet<Location>) -> Result<Option<(Location, WaveFunction)>, ()> {
        
        let mut lowest: Option<(Location, WaveFunction)> = None;
        let mut lowest_entropy = Entropy::MAX;

        let mut all_visited = true;
        let mut all_certain = true;

        for i in 0..CELL_COUNT {

            if let Cell::Uncertain { wave } = self.get_index(i) {

                all_certain = false;

                let location = Location::from_index(i);

                // If this cell was already visited, skip it
                if visited_cells.contains(&location) {
                    continue;
                }
                
                let local_entropy = wave.entropy();

                // 2 is the lowest valid entropy a cell can have.
                // If a cell has entrpy 2, then it's guaranteed that no other cell has a lower entropy value (except those that are already determinate).
                // Stop the search here to avoid useless iterations.
                if local_entropy == 2 {
                    return Ok(Some((location, wave)));
                }
                
                if local_entropy >= lowest_entropy {
                    continue;
                }

                // Set this flag here, after the entropy check, to avoid setting it for every unvisited cell.
                // If there is at least one uncertain and unvisited cell, this line will be executed at least once.
                all_visited = false;

                lowest = Some((location, wave));
                lowest_entropy = local_entropy;
            }
        }

        if all_certain {
            Ok(None)
        } else if all_visited {
            Err(())
        } else {
            Ok(lowest)
        }
    }

}


#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn check_blank_cells() {

        let board = Grid::new_random();

        let cap = 30;
        
        let with_blanks = board.with_random_blank_cells(cap);

        let blanks = with_blanks.cells.iter().filter(|cell| matches!(cell, Cell::Blank)).count();

        assert!(cap >= blanks as u8);
    }

}

