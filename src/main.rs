mod grid;
mod config;
mod solver;
mod cli_parser;
mod file_utils;


use clap::Parser;
use cli_parser::{CliParser, Commands};
use config::DEFAULT_BLANK_CELL_CAP;
use grid::Grid;
use file_utils::{load_board, save_board};


fn main() {

    let args = CliParser::parse();

    match args.command {
        
        Commands::View { input_file } => {
            
            let board = load_board(&input_file);
            
            println!("{board}");

        },

        Commands::Gen { output_file, solve, save_solution, solving_algorithm, blank_cell_cap } => {
            
            let board = Grid::new_random()
                .with_random_blank_cells(blank_cell_cap.unwrap_or(DEFAULT_BLANK_CELL_CAP));

            if let Some(output_file) = output_file {
                save_board(&board, &output_file);
            } else {
                println!("{board}");
            }

            if solve {

                let solved_board = solver::solve_with(&board, solving_algorithm);

                if let Some(output_file) = save_solution {
                    save_board(&solved_board, &output_file);
                } else {
                    println!("{solved_board}");
                }
            }

        },

        Commands::Solve { input_file, output_file, solving_algorithm } => {
            
            let board = load_board(&input_file);

            println!("{board}");

            let solved_board = solver::solve_with(&board, solving_algorithm);

            if let Some(output_file) = output_file {
                save_board(&solved_board, &output_file);
            } else {
                println!("{solved_board}");
            }

        },

    }

}

