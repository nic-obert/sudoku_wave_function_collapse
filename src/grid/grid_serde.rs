use std::mem::{self, MaybeUninit};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::SerializeTuple;
use serde::de::{self, Visitor};

use crate::grid::{Cell, Grid};
use crate::config::CELL_COUNT;

use super::grid::CellsType;


impl Serialize for Grid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {

        // let cells = self.cells

        todo!()
    }
}


impl<'de> Deserialize<'de> for Grid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        todo!()
    }
}


pub fn serialize_cells<S>(cells: &CellsType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    let mut tup = serializer.serialize_tuple(CELL_COUNT)?;

    for cell in cells.iter() {
        tup.serialize_element(cell)?;
    }

    tup.end()
}


pub fn deserialize_cells<'de, D>(deserializer: D) -> Result<CellsType, D::Error> 
    where
        D: Deserializer<'de>
{

    struct CellVisitor;

    impl<'de> Visitor<'de> for CellVisitor {
        type Value = CellsType;
    
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_fmt(format_args!("an array of {CELL_COUNT} cells"))
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>, 
        {
            let mut cells = Box::new([MaybeUninit::<Cell>::uninit(); CELL_COUNT]);

            for i in 0..CELL_COUNT {

                let cell = seq.next_element()?.ok_or_else(
                    || de::Error::custom(format!("Expected {CELL_COUNT} cells, found {i}"))
                )?;

                cells[i] = MaybeUninit::new(cell);
            }

            Ok(
                Box::into_pin(unsafe {
                    mem::transmute::<Box<[MaybeUninit<Cell>; CELL_COUNT]>, Box<[Cell; CELL_COUNT]>>(cells)
                })
            )
        }
    }

    deserializer.deserialize_tuple(CELL_COUNT, CellVisitor)
}

