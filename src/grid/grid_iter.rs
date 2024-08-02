use std::collections::HashSet;

use crate::config::{CELLS_IN_COLUMN_PER_BOX, CELLS_IN_ROW_PER_BOX, CELLS_PER_SECTOR};

use super::{Location, Vec2};


pub struct RowIterator {
    current: Option<Location>
}

impl Iterator for RowIterator {
    type Item = Location;

    fn next(&mut self) -> Option<Self::Item> {

        if let Some(current) = self.current {
            self.current = current.right();
            Some(current)
        } else {
            None
        }
    }
}

impl RowIterator {

    pub fn new(center: Location) -> Self {
        Self {
            current: Some(Location {
                row: center.row,
                column: 0
            })
        }
    }

}


pub struct RowsIterator {
    current: Option<(Location, RowIterator)>
}

impl Iterator for RowsIterator {
    type Item = RowIterator;

    fn next(&mut self) -> Option<Self::Item> {
        
        if let Some((leftmost, row_iter)) = self.current.take() {
            
            self.current = leftmost.below()
                .map(|leftmost|
                    (
                        leftmost,
                        RowIterator::new(leftmost)
                    )
            );

            Some(row_iter)
        } else {
            None
        }
    }
}

impl RowsIterator {

    pub fn new() -> Self {

        let start = Location {
            row: 0,
            column: 0
        };

        Self {
            current: Some((
                start,
                RowIterator::new(start)
            ))
        }
    }

}


pub struct ColumnsIterator {
    current: Option<(Location, ColumnIterator)>
}

impl Iterator for ColumnsIterator {
    type Item = ColumnIterator;

    fn next(&mut self) -> Option<Self::Item> {
        
        if let Some((topmost, column_iter)) = self.current.take() {
            
            self.current = topmost.right()
                .map(|leftmost| 
                    (
                        leftmost,
                        ColumnIterator::new(topmost)
                    )
            );

            Some(column_iter)
        } else {
            None
        }
    }
}

impl ColumnsIterator {

    pub fn new() -> Self {

        let start = Location {
            row: 0,
            column: 0
        };

        Self {
            current: Some((
                start,
                ColumnIterator::new(start)
            ))
        }
    }

}


pub struct BoxesIterator {
    current: Option<(Location, BoxIterator)>
}

impl Iterator for BoxesIterator {
    type Item = BoxIterator;

    fn next(&mut self) -> Option<Self::Item> {
    
        if let Some((topleft, box_iter)) = self.current.take() {

            self.current = if let Some(next_topleft) = topleft.add(Vec2 {
                                                                    rows: 0,
                                                                    columns: CELLS_IN_ROW_PER_BOX as i8
                                                                })
            {
                Some((
                    next_topleft,
                    BoxIterator::new(next_topleft)
                ))
            } else if let Some(next_topleft) = topleft.add(Vec2 {
                                                            rows: CELLS_IN_COLUMN_PER_BOX as i8,
                                                            columns: -(CELLS_IN_ROW_PER_BOX as i8 * 2)
                                                        })
            {
                Some((
                    next_topleft,
                    BoxIterator::new(next_topleft)
                ))
            } else {
                None
            };

            Some(box_iter)
        } else {
            None
        }
    }
}

impl BoxesIterator {
    
    pub fn new() -> Self {

        let topleft = Location {
            row: 0,
            column: 0
        };

        Self {
            current: Some((
                topleft,
                BoxIterator::new(topleft)
            ))
        }
    }

}


pub struct ColumnIterator {
    current: Option<Location>
}

impl Iterator for ColumnIterator {
    type Item = Location;

    fn next(&mut self) -> Option<Self::Item> {

        if let Some(current) = self.current {
            self.current = current.below();
            Some(current)
        } else {
            None
        }
    }
}

impl ColumnIterator {

    pub fn new(center: Location) -> Self {
        Self {
            current: Some(Location {
                row: 0,
                column: center.column
            })
        }
    }

}


pub struct BoxIterator {
    top_left: Location,
    row: u8,
    column: u8,
    done: bool
}

impl BoxIterator {

    pub fn new(center: Location) -> Self {
        Self {
            top_left: Location {
                row: center.row / CELLS_IN_COLUMN_PER_BOX as u8 * CELLS_IN_COLUMN_PER_BOX as u8,
                column: center.column / CELLS_IN_ROW_PER_BOX as u8 * CELLS_IN_ROW_PER_BOX as u8
            }, 
            row: 0, 
            column: 0,
            done: false
        }
    }

}

impl Iterator for BoxIterator {
    type Item = Location;

    fn next(&mut self) -> Option<Self::Item> {

        if self.done {
            return None;
        }

        let current = unsafe {
            self.top_left.add_unchecked(Vec2 { 
                rows: self.row as i8,
                columns: self.column as i8 
            })
        };

        if self.column == CELLS_IN_ROW_PER_BOX as u8 - 1 {

            if self.row == CELLS_IN_COLUMN_PER_BOX as u8 - 1 {
                self.done = true;
            } else {
                self.row += 1;
                self.column = 0;
            }
        } else {
            self.column += 1;
        }

        Some(current)
    }
}


pub struct SectorIterator {

    visited: HashSet<Location>,
    iter_mode: SectorIterMode,
    center: Location

}


enum SectorIterMode {

    Row (RowIterator),

    Column (ColumnIterator),

    Box (BoxIterator),

}

impl SectorIterator {

    pub fn new(center: Location) -> Self {
        Self {
            visited: HashSet::with_capacity(CELLS_PER_SECTOR),
            iter_mode: SectorIterMode::Row(RowIterator::new(center)),
            center
        }
    }

}

impl Iterator for SectorIterator {
    type Item = Location;

    fn next(&mut self) -> Option<Self::Item> {

        /*
            Iteration goes like this:
            1. iterate through the row
            2. iterate through the column
            3. iterate through the box
        */
        
        match &mut self.iter_mode {

            SectorIterMode::Row (row_iter) => {

                if let Some(next) = row_iter.next() {

                    if self.visited.insert(next) {
                        Some(next)
                    } else {
                        self.next()
                    }

                } else {
                    self.iter_mode = SectorIterMode::Column (ColumnIterator::new(self.center));
                    self.next()
                }
            },

            SectorIterMode::Column (column_iter) => {

                if let Some(next) = column_iter.next() {

                    if self.visited.insert(next) {
                        Some(next)
                    } else {
                        self.next()
                    }

                } else {
                    self.iter_mode = SectorIterMode::Box (BoxIterator::new(self.center));
                    self.next()
                }
            },

            SectorIterMode::Box (box_iter) => {

                if let Some(next) = box_iter.next() {

                    if self.visited.insert(next) {
                        Some(next)
                    } else {
                        self.next()
                    }

                } else {
                    None
                }
            }
        }
    }
}


pub fn iter_sector(location: Location) -> SectorIterator {

    SectorIterator::new(location)
}


pub fn iter_rows() -> RowsIterator {

    RowsIterator::new()
}


pub fn iter_columns() -> ColumnsIterator {

    ColumnsIterator::new()
}


pub fn iter_boxes() -> BoxesIterator {

    BoxesIterator::new()
}


#[allow(dead_code)]
pub fn iter_row(location: Location) -> RowIterator {

    RowIterator::new(location)
}


#[allow(dead_code)]
pub fn iter_column(location: Location) -> ColumnIterator {

    ColumnIterator::new(location)
}


#[allow(dead_code)]
pub fn iter_box(location: Location) -> BoxIterator {

    BoxIterator::new(location)
}



#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use crate::grid::Location;
    use crate::config::{CELLS_PER_BOX, CELLS_PER_COLUMN, CELLS_PER_ROW, CELLS_PER_SECTOR};
    
    use super::*;


    #[test]
    fn check_sector_iterator() {

        let mut visited: HashSet<Location> = HashSet::new();

        let start = Location {
            row: 0,
            column: 1
        };

        for cell in iter_sector(start) {
            println!("{cell}");
            assert!(
                visited.insert(cell)
            );
        }

        assert_eq!(visited.len(), CELLS_PER_SECTOR);

    }


    #[test]
    fn check_sector_iterator_order() {

        let start = Location {
            row: 0,
            column: 1
        };

        let mut it = iter_sector(start);

        assert_eq!(Some(Location { row: 0, column: 0 }), it.next());
        assert_eq!(Some(Location { row: 0, column: 1 }), it.next());
        assert_eq!(Some(Location { row: 0, column: 2 }), it.next());
        assert_eq!(Some(Location { row: 0, column: 3 }), it.next());
        assert_eq!(Some(Location { row: 0, column: 4 }), it.next());
        assert_eq!(Some(Location { row: 0, column: 5 }), it.next());
        assert_eq!(Some(Location { row: 0, column: 6 }), it.next());
        assert_eq!(Some(Location { row: 0, column: 7 }), it.next());
        assert_eq!(Some(Location { row: 0, column: 8 }), it.next());
        assert_eq!(Some(Location { row: 1, column: 1 }), it.next());
        assert_eq!(Some(Location { row: 2, column: 1 }), it.next());
        assert_eq!(Some(Location { row: 3, column: 1 }), it.next());
        assert_eq!(Some(Location { row: 4, column: 1 }), it.next());
        assert_eq!(Some(Location { row: 5, column: 1 }), it.next());
        assert_eq!(Some(Location { row: 6, column: 1 }), it.next());
        assert_eq!(Some(Location { row: 7, column: 1 }), it.next());
        assert_eq!(Some(Location { row: 8, column: 1 }), it.next());
        assert_eq!(Some(Location { row: 1, column: 0 }), it.next());
        assert_eq!(Some(Location { row: 1, column: 2 }), it.next());
        assert_eq!(Some(Location { row: 2, column: 0 }), it.next());
        assert_eq!(Some(Location { row: 2, column: 2 }), it.next());
        assert!(it.next().is_none());

    }


    #[test]
    fn check_row_iterator() {

        let mut visited: HashSet<Location> = HashSet::new();

        let start = Location {
            row: 0,
            column: 1
        };

        for cell in iter_row(start) {
            println!("{cell}");
            assert!(
                visited.insert(cell)
            );
        }

        assert_eq!(visited.len(), CELLS_PER_ROW);
        
    }


    #[test]
    fn check_row_iterator_order() {

        let start = Location {
            row: 0,
            column: 1
        };

        let mut it = iter_row(start);

        assert_eq!(Some(Location { row: 0, column: 0 }), it.next());
        assert_eq!(Some(Location { row: 0, column: 1 }), it.next());
        assert_eq!(Some(Location { row: 0, column: 2 }), it.next());
        assert_eq!(Some(Location { row: 0, column: 3 }), it.next());
        assert_eq!(Some(Location { row: 0, column: 4 }), it.next());
        assert_eq!(Some(Location { row: 0, column: 5 }), it.next());
        assert_eq!(Some(Location { row: 0, column: 6 }), it.next());
        assert_eq!(Some(Location { row: 0, column: 7 }), it.next());
        assert_eq!(Some(Location { row: 0, column: 8 }), it.next());
        assert!(it.next().is_none());

    }


    #[test]
    fn check_column_iterator() {

        let mut visited: HashSet<Location> = HashSet::new();

        let start = Location {
            row: 0,
            column: 1
        };

        for cell in iter_column(start) {
            println!("{cell}");
            assert!(
                visited.insert(cell)
            );
        }

        assert_eq!(visited.len(), CELLS_PER_COLUMN);
        
    }


    #[test]
    fn check_column_iterator_order() {

        let start = Location {
            row: 0,
            column: 1
        };

        let mut it = iter_column(start);

        assert_eq!(Some(Location { row: 0, column: 1 }), it.next());
        assert_eq!(Some(Location { row: 1, column: 1 }), it.next());
        assert_eq!(Some(Location { row: 2, column: 1 }), it.next());
        assert_eq!(Some(Location { row: 3, column: 1 }), it.next());
        assert_eq!(Some(Location { row: 4, column: 1 }), it.next());
        assert_eq!(Some(Location { row: 5, column: 1 }), it.next());
        assert_eq!(Some(Location { row: 6, column: 1 }), it.next());
        assert_eq!(Some(Location { row: 7, column: 1 }), it.next());
        assert_eq!(Some(Location { row: 8, column: 1 }), it.next());
        assert!(it.next().is_none());

    }


    #[test]
    fn check_box_iterator() {

        let mut visited: HashSet<Location> = HashSet::new();

        let start = Location {
            row: 0,
            column: 1
        };

        for cell in iter_box(start) {
            println!("{cell}");
            assert!(
                visited.insert(cell)
            );
        }

        assert_eq!(visited.len(), CELLS_PER_BOX);
        
    }


    #[test]
    fn check_box_iterator_order() {

        let start = Location {
            row: 0,
            column: 1
        };

        let mut it = iter_box(start);

        assert_eq!(Some(Location { row: 0, column: 0 }), it.next());
        assert_eq!(Some(Location { row: 0, column: 1 }), it.next());
        assert_eq!(Some(Location { row: 0, column: 2 }), it.next());
        assert_eq!(Some(Location { row: 1, column: 0 }), it.next());
        assert_eq!(Some(Location { row: 1, column: 1 }), it.next());
        assert_eq!(Some(Location { row: 1, column: 2 }), it.next());
        assert_eq!(Some(Location { row: 2, column: 0 }), it.next());
        assert_eq!(Some(Location { row: 2, column: 1 }), it.next());
        assert_eq!(Some(Location { row: 2, column: 2 }), it.next());
        assert!(it.next().is_none());

    }

}