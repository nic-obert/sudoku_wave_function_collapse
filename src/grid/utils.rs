

#[macro_export]
macro_rules! parse_cell {
    ($digit:literal) => {
        Cell::Certain { digit: $digit }
    };
    (($($state:literal),+)) => {
        Cell::Uncertain { 
            wave: WaveFunction::new_possibilities(&[$($state),+])
        }
    };
    (()) => {
        Cell::Blank
    }
}


#[macro_export]
macro_rules! create_board {
    ([$(
        $cell: tt
    ),+]) => {{
        use crate::parse_cell;
        use crate::grid::{Grid, Cell};

        let cells = Box::new([
            $(
                parse_cell!($cell)
            ),+
        ]);
        Grid::new_from_cells(
            cells.try_into().unwrap()
        )
    }};
}

