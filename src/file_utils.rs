use std::path::Path;
use std::fs;

use crate::grid::Grid;


fn error(message: &str) -> ! {
    eprintln!("{message}");
    std::process::exit(1);
}


pub fn save_board<P>(board: &Grid, file_path: &P)
    where
        P: AsRef<Path> + ?Sized
{
    let output = board.serialize_to_string();

    fs::write(file_path, output).unwrap_or_else(
        |e| error(format!("Could not save file {}:\n{}", file_path.as_ref().display(), e).as_str())
    );
}


pub fn load_board<P>(file_path: &P) -> Grid 
    where
        P: AsRef<Path> + ?Sized
{
    let raw_data = fs::read_to_string(&file_path).unwrap_or_else(
        |e| error(format!("Could not read file {}:\n{}", file_path.as_ref().display(), e).as_str())
    );

    let board: Grid = Grid::deserialize_from_string(&raw_data).unwrap_or_else(
        |e| error(format!("Could not parse file {}:\n{}", file_path.as_ref().display(), e).as_str())
    );

    board
}

