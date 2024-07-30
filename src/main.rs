mod location;
mod grid;
mod cell;
mod config;
mod solver;
mod cli_parser;
mod grid_iter;


use std::{fs, path::Path};

use clap::Parser;
use cli_parser::{CliParser, Commands};
use config::DEFAULT_HINT_COUNT;
use grid::Grid;


fn error(message: &str) -> ! {
    eprintln!("{message}");
    std::process::exit(1);
}


fn save_board(board: &Grid, file_path: &Path) {
    
    let json = serde_json::to_string(&board).expect("A generated board should be serializable");

    fs::write(file_path, json).unwrap_or_else(
        |e| error(format!("Could not save file {}:\n{}", file_path.display(), e).as_str())
    );
}


fn load_board(file_path: &Path) -> Grid {

    let raw_data = fs::read_to_string(&file_path).unwrap_or_else(
        |e| error(format!("Could not read file {}:\n{}", file_path.display(), e).as_str())
    );

    let board: Grid = serde_json::from_str(&raw_data).unwrap_or_else(
        |e| error(format!("Could not parse file {}:\n{}", file_path.display(), e).as_str())
    );

    board
}


fn main() {

    let args = CliParser::parse();

    match args.command {
        
        Commands::View { input_file } => {
            
            let board = load_board(&input_file);

            println!("{board}");

        },

        Commands::Gen { output_file, solve, hints, save_solution } => {
            
            let board = Grid::new_random()
                .with_random_blank_cells(hints.unwrap_or(DEFAULT_HINT_COUNT));

            if let Some(output_file) = output_file {
                save_board(&board, &output_file);
            } else {
                println!("{board}");
            }

            if solve {

                let solved_board = solver::bruteforce_backtracking::solve(&board);

                if let Some(output_file) = save_solution {
                    save_board(&solved_board, &output_file);
                } else {
                    println!("{solved_board}");
                }
            }

        },

        Commands::Solve { input_file, output_file } => {
            
            let board = load_board(&input_file);

            let solved_board = solver::bruteforce_backtracking::solve(&board);

            if let Some(output_file) = output_file {
                save_board(&solved_board, &output_file);
            } else {
                println!("{solved_board}");
            }

        },

    }

}

