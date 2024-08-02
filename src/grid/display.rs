use core::fmt;

use colored::Colorize;

use crate::config::{BOXES_PER_ROW, CELLS_IN_COLUMN_PER_BOX, CELLS_IN_ROW_PER_BOX, DIGIT_BASE, ROW_COUNT};

use super::Grid;


impl Grid {

    fn display_row(&self, row_index: usize, row_in_cell_index: usize, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "{}", "|".bold())?;

        for box_index in 0..BOXES_PER_ROW {

            write!(f, "{}", "|".bold())?;
            
            for cell_column_index in 0..CELLS_IN_ROW_PER_BOX {

                self.get_index(cell_column_index + box_index * CELLS_IN_ROW_PER_BOX + row_index * DIGIT_BASE)
                    .display_row(row_in_cell_index, f)?;

                write!(f, "{}", "|".bold())?;

            }

        }

        writeln!(f, "{}", "|".bold())
    }

}


fn display_horizontal_box_separator(f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "{}", "||=========================================================================||".bold())
}


fn display_horizontal_normal_separator(f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "{}", "||-------------------------------------------------------------------------||".bold())
}


impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        display_horizontal_box_separator(f)?;

        for row_index in 0..ROW_COUNT {

            for row_in_cell_index in 0..CELLS_IN_COLUMN_PER_BOX {

                self.display_row(row_index, row_in_cell_index, f)?;

            }

            if (row_index+1) % CELLS_IN_COLUMN_PER_BOX == 0 {
                display_horizontal_box_separator(f)?;
            } else {
                display_horizontal_normal_separator(f)?;
            }

        }

        Ok(())
    }
}

