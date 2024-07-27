mod location;
mod grid;
mod cell;
mod config;


fn main() {
   
    let board = grid::Grid::new_random();    

    println!("{}", board);

}

