mod location;
mod grid;
mod cell;
mod config;
mod solver;
mod cli_parser;
mod grid_iter;
mod file_utils;


use clap::Parser;
use cli_parser::{CliParser, Commands};
use config::DEFAULT_HINT_COUNT;
use grid::Grid;
use file_utils::{load_board, save_board};


fn main() {

    let args = CliParser::parse();

    match args.command {
        
        Commands::View { input_file } => {
            
            let board = load_board(&input_file);

            println!("{board}");

        },

        Commands::Gen { output_file, solve, hints, save_solution, solving_algorithm } => {
            
            let board = Grid::new_random()
                .with_random_blank_cells(hints.unwrap_or(DEFAULT_HINT_COUNT));

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

