use std::mem::{self, MaybeUninit};

use crate::grid::{Cell, Grid, WaveFunction};
use crate::config::CELL_COUNT;


impl Grid {

    pub fn serialize_to_string(&self) -> String {
        
        // TODO: very much not efficient to instantiate a string for every token, but it's concise and easy

        self.cells.iter()

            .map(|cell| match cell {

                Cell::Certain { digit } => 
                    digit.to_string(),

                Cell::Uncertain { wave }
                    => format!("({})", wave.states().map(|s| s.to_string()).collect::<Vec<String>>().join(",")),

                Cell::Blank
                    => "()".to_owned(),
            })

            .collect::<Vec<String>>()

            .join(",")
    }


    pub fn deserialize_from_string(input: &str) -> Result<Self, String> {

        // This parser allows a narrow superset of the intended language, but it's ok because it works and I'm lazy

        let mut cells = Box::new([MaybeUninit::<Cell>::uninit(); CELL_COUNT]);

        let mut cell_i = 0;

        enum ParserState {
            TopLevel,
            InCell (WaveFunction)
        }

        let mut state = ParserState::TopLevel;
        let mut last_ch = ',';

        let mut input_iter = input.char_indices();

        while let Some((ch_i, ch)) = input_iter.next() {

            match ch {

                '(' => {

                    if matches!(state, ParserState::InCell (_)) {
                        return Err(
                            format!("Unexpected token '(' inside a cell descriptor at char index {ch_i}")
                        )
                    }

                    if !matches!(last_ch, ',' | ' ' | '\t' | '\n') {
                        return Err(
                            format!("Unexpected token '(' at char index {ch_i} after token '{last_ch}'")
                        )
                    }

                    state = ParserState::InCell (WaveFunction::new_min_entropy());
                },

                ')' => {

                    if last_ch == ',' {
                        return Err(
                            format!("Unexpected token ')' at char index {ch_i} after token ','")
                        );
                    }

                    match state {

                        ParserState::TopLevel
                            => return Err(
                                format!("Unexpected token ')' outside a cell descriptor at char index {ch_i}")
                            ),

                        ParserState::InCell(wave) => {

                            cells[cell_i] = MaybeUninit::new(
                                if wave.entropy() == 0 {
                                    Cell::Blank
                                } else {
                                    Cell::Uncertain { wave }
                                }
                            );

                            state = ParserState::TopLevel;

                            cell_i += 1;

                            if cell_i == CELL_COUNT {
                                break;
                            }
                        }
                    }
                    
                },
                
                ',' => if matches!(last_ch, ',' | '(') {
                        return Err(
                            format!("Unexpected token ',' at char index {ch_i} after token '{last_ch}'")
                        );
                    },

                ch => {
                    if let Some(digit) = ch.to_digit(10) {

                        if digit == 0 {
                            return Err(
                                format!("Unexpected digit '0' as cell state. Possible states are digits 1-9")
                            )
                        }

                        match state {

                            ParserState::TopLevel => {
                                cells[cell_i] = MaybeUninit::new(Cell::Certain { digit: digit as u8 });

                                cell_i += 1;

                                if cell_i == CELL_COUNT {
                                    break;
                                }
                            },

                            ParserState::InCell(mut wave) => {
                                wave.add_possibility(digit as u8);
                                state = ParserState::InCell(wave)
                            }
                        }

                    } else if !matches!(ch, ' ' | '\t' | '\n') {
                        return Err(
                            format!("Unexpected token '{ch}' at char index {ch_i}")
                        )
                    }
                }
            }

            last_ch = ch;
        }

        if matches!(state, ParserState::InCell(_)) {
            return Err(
                format!("Unclosed cell descriptor")
            )
        }

        while let Some((ch_i, ch)) = input_iter.next() {
            if !matches!(ch, ' ' | '\t' | '\n') {
                return Err(
                    format!("Unexpected token '{ch}' at char index {ch_i}. All {CELL_COUNT} cells have been provided.")
                );
            }
        } 

        if cell_i != CELL_COUNT {
            return Err(
                format!("Too few cells were provided: expected {CELL_COUNT} cells, but got {} cells", cell_i - 1)
            );
        }

        Ok(Self {
            cells: unsafe {
                mem::transmute(cells)
            }
        })
    }

}

