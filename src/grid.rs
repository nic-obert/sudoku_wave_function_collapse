mod grid;
mod display;
mod utils;
mod grid_serde;
mod grid_functions;
mod cell;
mod location;
mod grid_iter;


pub use grid::Grid;
pub use location::*;
pub use cell::*;
pub use grid_iter::*;


#[cfg(test)]
mod tests {

    use crate::grid::Grid;
    use crate::grid::WaveFunction;
    use crate::create_board;

    #[test]
    fn wave_function_random_collapse() {

        let rng = rand::thread_rng();

        let wave = WaveFunction::new_possibilities(
            &[1,3,5,8,9]
        );

        assert!(wave.collapse_random(rng.clone()).is_some());

        let empty_wave = WaveFunction::new_possibilities(
            &[]
        );

        assert!(empty_wave.collapse_random(rng.clone()).is_none());
    }


    #[test]
    fn check_valid_generation() {

        for _ in 0..1000 {
            assert!(Grid::new_random().check_valid());
        }
    }


    #[test]
    fn create_board() {

        let board = create_board!([
            1,2,3,4,5,6,7,8,9,
            1,2,3,4,5,6,7,8,9,
            1,2,3,4,5,6,7,8,9,
            1,2,3,4,5,6,7,8,9,
            1,2,3,4,5,6,7,8,9,
            1,2,3,4,5,6,7,8,9,
            1,2,3,4,5,6,7,8,9,
            1,2,3,4,5,6,7,8,9,
            1,2,3,4,5,6,7,8,9
        ]);

        assert!(!board.check_valid());

        let board = create_board!([
            5,3,4,6,7,8,9,1,2,
            6,7,2,1,9,5,3,4,8,
            1,9,8,3,4,2,5,6,7,
            8,5,9,7,6,1,4,2,3,
            4,2,6,8,5,3,7,9,1,
            7,1,3,9,2,4,8,5,6,
            9,6,1,5,3,7,2,8,4,
            2,8,7,4,1,9,6,3,5,
            3,4,5,2,8,6,1,7,9
        ]);

        assert!(board.check_valid());

        let board = create_board!([
            5,(3),4,6,7,8,9,1,2,
            6,7,(5,3,1),1,9,5,3,4,8,
            1,9,8,3,4,(1,3,6),5,6,7,
            8,5,9,7,6,1,4,2,3,
            4,2,6,8,5,3,7,9,1,
            7,1,3,9,2,4,8,5,6,
            9,6,1,5,3,7,2,8,4,
            2,8,7,4,1,9,6,3,5,
            3,4,5,2,8,6,1,7,9
        ]);

        assert!(board.check_valid());

    }
    
}

