use core::fmt;

use crate::config::{COLUMN_COUNT, DIGIT_BASE, ROW_COUNT};


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Location {

    pub row: u8,
    pub column: u8

}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.column)
    }
}

impl Location {

    pub const fn from_index(i: usize) -> Self {
        Self {
            row: (i / DIGIT_BASE) as u8,
            column: (i % DIGIT_BASE) as u8
        }
    }


    pub const fn into_index(self) -> usize {
        self.column as usize + self.row as usize * COLUMN_COUNT
    }


    pub const unsafe fn add_unchecked(self, v: Vec2) -> Self {
        Self {
            row: (self.row as i8 + v.rows) as u8,
            column: (self.column as i8 + v.columns) as u8
        }
    }


    pub const fn add(self, v: Vec2) -> Option<Self> {

        let row = self.row as i8 + v.rows;
        let column = self.column as i8 + v.columns;

        if row < 0 || row >= ROW_COUNT as i8 || column < 0 || column >= COLUMN_COUNT as i8 {
            None 
        } else {
            Some(Self {
                row: row as u8,
                column: column as u8
            })
        }
    }


    // #[inline]
    // pub const fn above(self) -> Option<Self> {
    //     self.add(Vec2 { rows: -1, columns: 0 })
    // }

    #[inline]
    pub const fn below(self) -> Option<Self> {
        self.add(Vec2 { rows: 1, columns: 0 })
    }

    // #[inline]
    // pub const fn left(self) -> Option<Self> {
    //     self.add(Vec2 { rows: 0, columns: -1 })
    // }

    #[inline]
    pub const fn right(self) -> Option<Self> {
        self.add(Vec2 { rows: 0, columns: 1 })
    }

}


pub struct Vec2 {
    pub rows: i8,
    pub columns: i8
}

