use core::fmt;
use std::collections::HashSet;
use std::u8;

use rand::{thread_rng, Rng};
use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::location::Location;
use crate::config::{BOXES_PER_ROW, CELL_COUNT, DIGITS_IN_COLUMN_PER_BOX, DIGITS_IN_ROW_PER_BOX, DIGIT_BASE, ROW_COUNT};
use crate::cell::{Cell, Digit, Entropy, WaveFunction};


#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Grid {

    #[serde(with = "serde_arrays")]
    cells: [Cell; CELL_COUNT]

}

impl Grid {

    pub fn new_max_entropy() -> Self {
        Self {
            cells: [Cell::new_max_entropy(); CELL_COUNT]
        }
    }


    #[inline]
    pub fn get_at(&self, location: Location) -> Cell {
        self.cells[location.into_index()]
    }


    #[inline]
    pub fn set_at(&mut self, location: Location, cell: Cell) {
        self.cells[location.into_index()] = cell;
    }


    #[inline]
    pub fn set_index(&mut self, i: usize, cell: Cell) {
        self.cells[i] = cell;
    }


    #[inline]
    pub fn get_index(&self, i: usize) -> Cell {
        self.cells[i]
    }


    pub fn get_sector(&self, location: Location) -> HashSet<Location> {

        // A capacity of 21 is exactly the size of a sector
        // A hashset may not be the most efficient data structure for this application due to hashing time, though
        let mut sector = HashSet::with_capacity(21);

        // Search in column

        let mut current_in_column = Location {
            row: 0,
            column: location.column
        };

        sector.insert(current_in_column);
        
        while let Some(below) = current_in_column.below() {
            current_in_column = below;
            sector.insert(current_in_column);
        }

        // Search in the row

        let mut current_in_row = Location {
            row: location.row,
            column: 0
        };

        sector.insert(current_in_row);

        while let Some(right) = current_in_row.right() {
            current_in_row = right;
            sector.insert(current_in_row);
        }

        // Search in the box

        let mut leftmost = Location {
            row: location.row / DIGITS_IN_COLUMN_PER_BOX as u8 * DIGITS_IN_COLUMN_PER_BOX as u8,
            column: location.column / DIGITS_IN_ROW_PER_BOX as u8 * DIGITS_IN_ROW_PER_BOX as u8
        };

        for _ in 0..DIGITS_IN_COLUMN_PER_BOX {

            let mut current_in_box = leftmost;

            for _ in 0..DIGITS_IN_ROW_PER_BOX {

                sector.insert(current_in_box);

                current_in_box = if let Some(right) = current_in_box.right() {
                    right
                } else {
                    break
                };
            }

            leftmost = if let Some(below) = leftmost.below() {
                below
            } else {
                break
            };
        }

        sector
    }


    pub fn wave_at(&self, location: Location) -> WaveFunction {

        let mut wave = WaveFunction::new_max_entropy();

        for cell in self.get_sector(location) {

            // Ignore the compatibility of single wave functions in the sector 

            if let Cell::Certain { digit } = self.get_at(cell) {
                wave.remove_possibility(digit);
            }

        }

        wave
    }


    /// Collapse the specified cell and update all the cells in its sector accordingly. 
    /// Recursively collapse all cells that reach a collapsible state as a consequence of a previous collapse.
    /// This function fails if the sudoku rules are not satisfied after the collapse.
    pub fn update_collapse(&mut self, location: Location, collapsed_digit: Digit) -> Result<(), ()> {

        self.set_at(location, Cell::Certain { digit: collapsed_digit });
        
        for cell in self.get_sector(location) {
            
            match self.get_at(cell) {
                
                Cell::Certain { digit } => {
                    if cell != location && digit == collapsed_digit {
                        return Err(());
                    }
                },
                
                Cell::Uncertain { mut wave } => {
                    
                    wave.remove_possibility(collapsed_digit);
                    
                    if let Some(newly_collapsed) = wave.collapsed() {
                        // Recursively collapse all collapsible cells
                        self.update_collapse(cell, newly_collapsed)?;
                    } else {
                        self.set_at(cell, Cell::Uncertain { wave });
                    };
                },

                Cell::Blank => {
                    // Do nothing. Blank cells will be updated directly by the solver.
                }
            }

        }

        Ok(())
    }


    pub fn new_random() -> Self {

        let rng = thread_rng();

        'gen_attempt: loop {

            let mut grid = Self::new_max_entropy();

            for i in 0..CELL_COUNT {

                match grid.get_index(i) {

                    Cell::Uncertain { wave } => {

                        let collapsed = wave.collapse_random(rng.clone()).expect("Should be valid, but maybe it's not");
                        
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

                    for neighbor in self.get_sector(location) {

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


    pub fn with_random_blank_cells(&self, non_blank_cells: u8) -> Self {

        let mut to_clear = CELL_COUNT as u8 - non_blank_cells;
        let mut new_grid = self.clone();

        let mut rng = rand::thread_rng();

        while to_clear != 0 {

            let i = rng.gen_range(0..CELL_COUNT);
            
            if matches!(new_grid.get_index(i), Cell::Blank) {
                continue;
            }

            new_grid.set_index(i, Cell::Blank);

            to_clear -= 1;
        }

        new_grid
    }


    /// Return the cell with the lowest entropy among the uncertain cells, skipping the cells that were already visited.
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


    fn display_row(&self, row_index: usize, row_in_cell_index: usize, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "{}", "|".bold())?;

        for box_index in 0..BOXES_PER_ROW {

            write!(f, "{}", "|".bold())?;
            
            for cell_column_index in 0..DIGITS_IN_ROW_PER_BOX {

                self.get_index(cell_column_index + box_index * DIGITS_IN_ROW_PER_BOX + row_index * DIGIT_BASE)
                    .display_row(row_in_cell_index, f)?;

                write!(f, "{}", "|".bold())?;

            }

        }

        writeln!(f, "{}", "|".bold())
    }


    fn display_horizontal_box_separator(f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", "||=========================================================================||".bold())
    }


    fn display_horizontal_normal_separator(f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", "||-------------------------------------------------------------------------||".bold())
    }

}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        Grid::display_horizontal_box_separator(f)?;

        for row_index in 0..ROW_COUNT {

            for row_in_cell_index in 0..DIGITS_IN_COLUMN_PER_BOX {

                self.display_row(row_index, row_in_cell_index, f)?;

            }

            if (row_index+1) % DIGITS_IN_COLUMN_PER_BOX == 0 {
                Grid::display_horizontal_box_separator(f)?;
            } else {
                Grid::display_horizontal_normal_separator(f)?;
            }

        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn wave_function_random_collapse() {

        let rng = rand::thread_rng();

        let wave = WaveFunction::new_possibilities(
            &[1,3,5,8,9]
        );

        assert!(wave.collapse_random(rng.clone()).is_some());

        let empty_wave = WaveFunction::new_possibilities(
            &[]
        );

        assert!(empty_wave.collapse_random(rng.clone()).is_none());
    }


    #[test]
    fn check_valid_generation() {

        for _ in 0..1000 {
            assert!(Grid::new_random().check_valid());
        }
    }
    
}

