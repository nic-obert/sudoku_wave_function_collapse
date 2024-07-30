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
    match algorithm {
        SolvingAlgorithms::BruteforceBacktracking => bruteforce_backtracking::solve(board),
        SolvingAlgorithms::NeighboringWavesIntersection => neighboring_waves_intersection::solve(board),
    }
}

