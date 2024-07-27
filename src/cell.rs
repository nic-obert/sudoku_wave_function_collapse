use core::fmt;
use colored::Colorize;
use rand::Rng;

use crate::config::{CERTAIN_DIGIT_ROW_IN_BOX, DIGITS_IN_ROW_PER_CELL, DIGIT_BASE};


#[derive(Clone, Copy)]
pub enum Cell {

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
                
                const ASCII_DIGITS_BASE: usize = 48;

                write!(f, " {} {} {} ",
                    if wave.possibilities[0 + row_in_cell_index*DIGITS_IN_ROW_PER_CELL] { (1 + row_in_cell_index*DIGITS_IN_ROW_PER_CELL + ASCII_DIGITS_BASE) as u8 as char } else { ' ' },
                    if wave.possibilities[1 + row_in_cell_index*DIGITS_IN_ROW_PER_CELL] { (2 + row_in_cell_index*DIGITS_IN_ROW_PER_CELL + ASCII_DIGITS_BASE) as u8 as char } else { ' ' },
                    if wave.possibilities[2 + row_in_cell_index*DIGITS_IN_ROW_PER_CELL] { (3 + row_in_cell_index*DIGITS_IN_ROW_PER_CELL + ASCII_DIGITS_BASE) as u8 as char } else { ' ' },
                )
            }
        }
    }

}


/// Number in the range 1..=NUMBER_BASE
pub type Digit = u8;


#[derive(Clone, Copy)]
pub struct WaveFunction {

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

