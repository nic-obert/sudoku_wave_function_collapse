use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::solver::SolvingAlgorithms;


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
        #[arg(short = 'a', long, requires("solve"), value_enum, default_value_t)]
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

        /// Save the solved board to a file
        #[arg(short = 'f', long, requires("solve"))]
        save_solution: Option<PathBuf>,

        /// How many initial hints the generated Sudoku has
        #[arg(long)]
        hints: Option<u8>,

        /// Select the algorithm to solve the Sudoku
        #[arg(short = 'a', long, requires("solve"), value_enum, default_value_t)]
        solving_algorithm: SolvingAlgorithms

    }

}

