pub mod bruteforce_backtracking;
pub mod neighboring_waves_intersection;
mod utils;

use crate::grid::Grid;


#[derive(Clone, clap::ValueEnum, Default)]
pub enum SolvingAlgorithms {

    BruteforceBacktracking,
    #[default]
    NeighboringWavesIntersection,

}


pub fn solve_with(board: &Grid, algorithm: SolvingAlgorithms) -> Grid {

    let solved_board = match algorithm {
        SolvingAlgorithms::BruteforceBacktracking => bruteforce_backtracking::solve(board),
        SolvingAlgorithms::NeighboringWavesIntersection => neighboring_waves_intersection::solve(board),
    };

    assert!(solved_board.check_valid());

    solved_board
}


#[cfg(test)]
mod tests {

    use crate::file_utils;

    use super::{solve_with, SolvingAlgorithms};


    #[test]
    fn check_neighboring_waves_intersection() {

        for board in file_utils::get_test_boards() {
            solve_with(&board, SolvingAlgorithms::NeighboringWavesIntersection);
        }

    }

}

