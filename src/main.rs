use core::fmt;
use std::collections::HashSet;

use rand::Rng;
use colored::Colorize;


const ASCII_DIGITS_BASE: usize = 48;
const DIGITS_IN_ROW_PER_BOX: usize = 3;
const DIGITS_IN_COLUMN_PER_BOX: usize = DIGITS_IN_ROW_PER_BOX;
const BOXES_PER_ROW: usize = DIGITS_IN_ROW_PER_BOX;
const DIGITS_IN_ROW_PER_CELL: usize = BOXES_PER_ROW;
const CERTAIN_DIGIT_ROW_IN_BOX: usize = DIGITS_IN_ROW_PER_BOX / 2;
const DIGIT_BASE: usize = DIGITS_IN_ROW_PER_BOX * BOXES_PER_ROW;
const ROW_COUNT: usize = DIGIT_BASE;
const COLUMN_COUNT: usize = ROW_COUNT;
const CELL_COUNT: usize = DIGIT_BASE*DIGIT_BASE;


/// Number in the range 1..=NUMBER_BASE
type Digit = u8;


#[derive(Clone, Copy)]
struct WaveFunction {

    possibilities: [bool; DIGIT_BASE]

}

impl WaveFunction {

    pub const fn new_max_entropy() -> Self {
        Self {
            possibilities: [true; DIGIT_BASE]
        }
    }


    pub fn remove_possibility(&mut self, digit: Digit) {
        self.possibilities[digit as usize - 1] = false;
    }

    
    pub fn new_possibilities(possibilities: &[Digit]) -> Self {
        
        let mut wave = [false; DIGIT_BASE];

        for &digit in possibilities {
            // -1 because digits are 1-9 and arrays are 0-indexed
            wave[digit as usize - 1] = true;
        }

        Self {
            possibilities: wave
        }
    }


    pub fn collapse_random(self) -> Option<Digit> {
        
        let mut possibilities: [Digit; DIGIT_BASE] = [0; DIGIT_BASE];
        let mut possibility_i = 0;

        for (digit, &is_possible) in self.possibilities.iter().enumerate() {
            if is_possible {
                possibilities[possibility_i] = (digit+1) as Digit;
                possibility_i += 1;
            }
        }

        if possibility_i == 0 {
            return None;
        }

        let mut rng = rand::thread_rng();

        Some(
            possibilities[
                rng.gen_range(0..possibility_i)
            ]
        )
    }


    pub fn entropy(&self) -> u8 {

        let mut entropy = 0;

        for digit in self.possibilities {
            entropy += digit as u8;
        }

        entropy
    }


    pub fn collapsed(&self) -> Option<Digit> {

        let mut collapsed = None; 

        for (digit, &is_possible) in self.possibilities.iter().enumerate() {
            if is_possible {
                if collapsed.is_none() {
                    collapsed = Some(digit as Digit + 1);
                } else {
                    // More than one digit is possible, the wave function hasn't collapsed
                    return None;
                }
            }
        }

        collapsed
    }

}


#[derive(Clone, Copy)]
enum Cell {

    Certain { digit: Digit },
    Uncertain { wave: WaveFunction }

}

impl Cell {

    pub const fn new_max_entropy() -> Self {
        Self::Uncertain { 
            wave: WaveFunction::new_max_entropy()
        }
    }


    pub fn display_row(&self, row_in_cell_index: usize, f: &mut fmt::Formatter) -> fmt::Result {
        match self {

            Cell::Certain { digit } => {
                write!(f, "   {}   ", 
                    if row_in_cell_index == CERTAIN_DIGIT_ROW_IN_BOX {
                        digit.to_string().bold()
                    } else {
                        " ".into()
                    }
                )
            },

            Cell::Uncertain { wave } => {
                write!(f, " {} {} {} ",
                    if wave.possibilities[0 + row_in_cell_index*DIGITS_IN_ROW_PER_CELL] { (1 + row_in_cell_index*DIGITS_IN_ROW_PER_CELL + ASCII_DIGITS_BASE) as u8 as char } else { ' ' },
                    if wave.possibilities[1 + row_in_cell_index*DIGITS_IN_ROW_PER_CELL] { (2 + row_in_cell_index*DIGITS_IN_ROW_PER_CELL + ASCII_DIGITS_BASE) as u8 as char } else { ' ' },
                    if wave.possibilities[2 + row_in_cell_index*DIGITS_IN_ROW_PER_CELL] { (3 + row_in_cell_index*DIGITS_IN_ROW_PER_CELL + ASCII_DIGITS_BASE) as u8 as char } else { ' ' },
                )
            }
        }
    }

}


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Location {

    pub row: u8,
    pub column: u8

}

impl Location {

    pub const fn from_index(i: usize) -> Self {
        Self {
            row: (i / DIGIT_BASE) as u8,
            column: (i % DIGIT_BASE) as u8
        }
    }


    pub const fn into_index(self) -> usize {
        self.column as usize + self.row as usize * COLUMN_COUNT
    }


    pub const fn add(self, v: Vec2) -> Option<Self> {

        let row = self.row as i8 + v.rows;
        let column = self.column as i8 + v.columns;

        if row < 0 || row >= ROW_COUNT as i8 || column < 0 || column >= COLUMN_COUNT as i8 {
            None 
        } else {
            Some(Self {
                row: row as u8,
                column: column as u8
            })
        }
    }


    #[inline]
    pub const fn above(self) -> Option<Self> {
        self.add(Vec2 { rows: -1, columns: 0 })
    }

    #[inline]
    pub const fn below(self) -> Option<Self> {
        self.add(Vec2 { rows: 1, columns: 0 })
    }

    #[inline]
    pub const fn left(self) -> Option<Self> {
        self.add(Vec2 { rows: 0, columns: -1 })
    }

    #[inline]
    pub const fn right(self) -> Option<Self> {
        self.add(Vec2 { rows: 0, columns: 1 })
    }

}


struct Vec2 {
    rows: i8,
    columns: i8
}


struct Grid {

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
    pub fn get_index(&self, i: usize) -> Cell {
        self.cells[i]
    }


    pub fn get_sector(&self, location: Location) -> HashSet<Location> {

        // A capacity of 21 is exactly the size of a sector
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

        // For now, assume the cell at the specified location is uncertain
        // This may be changed later
        assert!(matches!(self.get_at(location), Cell::Uncertain { .. }));

        let mut wave = WaveFunction::new_max_entropy();

        for cell in self.get_sector(location) {

            if let Cell::Certain { digit } = self.get_at(cell) {
                wave.remove_possibility(digit);
            }

        }

        wave
    }


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
                    
                    // println!("{}", self);
                },
            }

        }

        Ok(())
    }


    pub fn new_random() -> Self {

        'gen_attempt: loop {

            let mut grid = Self::new_max_entropy();

            for i in 0..CELL_COUNT {

                // println!("\n\n{}", grid);
                
                match grid.get_index(i) {

                    Cell::Uncertain { wave } => {

                        let collapsed = wave.collapse_random().expect("Should be valid, but maybe it's not");
                        
                        if grid.update_collapse(Location::from_index(i), collapsed).is_err() {
                            continue 'gen_attempt;
                        }

                    },
                    
                    Cell::Certain { .. } => {
                        // Do nothing, the wave function is already collapsed
                    }
                }

            }

            assert!(grid.check_valid());

            return grid;
        }
    }


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

                Cell::Uncertain { .. } => {
                    // Do nothing, this is still uncertain
                },
            }

        }

        true
    }


    fn display_row(&self, row_index: usize, row_in_cell_index: usize, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "|")?;

        for box_index in 0..BOXES_PER_ROW {

            write!(f, "|")?;
            
            for cell_column_index in 0..DIGITS_IN_ROW_PER_BOX {

                self.get_index(cell_column_index + box_index * DIGITS_IN_ROW_PER_BOX + row_index * DIGIT_BASE)
                    .display_row(row_in_cell_index, f)?;

                write!(f, "|")?;

            }

        }

        writeln!(f, "|")
    }


    fn display_horizontal_box_separator(f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "||=========================================================================||")
    }


    fn display_horizontal_normal_separator(f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "||-------------------------------------------------------------------------||")
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


struct Board {

    grid: Grid

}

impl Board {
    
    pub fn new_random() -> Self {
        Self {
            grid: Grid::new_random()
        }
    }
    
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.grid)
    }
}


fn main() {

   
    let board = Board::new_random();    

    println!("{}", board);

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn wave_function_random_collapse() {

        let wave = WaveFunction::new_possibilities(
            &[1,3,5,8,9]
        );

        assert!(wave.collapse_random().is_some());

        let empty_wave = WaveFunction::new_possibilities(
            &[]
        );

        assert!(empty_wave.collapse_random().is_none());
    }


    #[test]
    fn check_valid_generation() {

        for _ in 0..1000 {
            Board::new_random();
        }
    }
    
}

