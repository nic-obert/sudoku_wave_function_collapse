use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::{config::CELL_COUNT, solver::SolvingAlgorithms};


#[derive(Parser)]
pub struct CliParser {

    #[command(subcommand)]
    pub command: Commands

}


#[derive(Subcommand)]
pub enum Commands {

    /// View the specified Sudoku file
    View {

        /// The Sudoku file to view
        input_file: PathBuf

    },

    /// Solve the specified Sudoku file
    Solve {

        /// The Sudoku file to solve
        input_file: PathBuf,

        /// Save the solved board to a file
        #[arg(short='o')]
        output_file: Option<PathBuf>,

        /// Select the algorithm to solve the Sudoku
        #[arg(short = 'a', long, value_enum, default_value_t)]
        solving_algorithm: SolvingAlgorithms

    },

    /// Generate a Sudoku board
    Gen {

        /// Save the generated board to a file
        #[arg(short='o')]
        output_file: Option<PathBuf>,

        /// Solve the generated Sudoku board
        #[arg(short='s')]
        solve: bool,

        /// Maximum number of blank cells.
        /// Note that the higher the blank cell cap, the longer it takes to generate the board
        #[arg(short='c', long, value_parser=blank_cell_cap_validator)]
        blank_cell_cap: Option<u8>,

        /// Save the solved board to a file
        #[arg(short = 'f', long, requires("solve"))]
        save_solution: Option<PathBuf>,

        /// Select the algorithm to solve the Sudoku
        #[arg(short = 'a', long, requires("solve"), value_enum, default_value_t)]
        solving_algorithm: SolvingAlgorithms

    }

}


fn blank_cell_cap_validator(s: &str) -> Result<u8, String> {
    clap_num::number_range(s, 0, CELL_COUNT as u8)
}

