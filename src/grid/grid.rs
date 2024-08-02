use std::pin::Pin;

use crate::config::CELL_COUNT;

use super::{Cell, Location};


pub type CellsType = Pin<Box<[Cell; CELL_COUNT]>>;


#[derive(Clone, PartialEq)]
pub struct Grid {

    pub(super) cells: CellsType

}

impl Grid {

    pub fn new_max_entropy() -> Self {
        Self {
            cells: Box::new([Cell::new_max_entropy(); CELL_COUNT]).into()
        }
    }


    #[allow(dead_code)]
    pub fn new_from_cells(cells: Box<[Cell; CELL_COUNT]>) -> Self {
        Self {
            cells: cells.into()
        }
    }


    #[inline]
    pub fn get_at(&self, location: Location) -> Cell {
        self.cells[location.into_index()]
    }


    #[inline]
    pub fn set_at(&mut self, location: Location, cell: Cell) {
        self.cells[location.into_index()] = cell;
    }


    #[inline]
    pub fn set_index(&mut self, i: usize, cell: Cell) {
        self.cells[i] = cell;
    }


    #[inline]
    pub fn get_index(&self, i: usize) -> Cell {
        self.cells[i]
    }

}

