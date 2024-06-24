use std::num::TryFromIntError;

use apis::{HexCoord, HexWorldShape};
use index::HexagonMapIndexer;

mod index;

pub struct Map<V> {
    vec: Vec<Option<V>>,
    indexer: HexagonMapIndexer,
}

impl<V: Copy> Map<V> {
    pub fn new(shape: HexWorldShape) -> Self {
        match shape {
            HexWorldShape::Hexagon(r) => {
                let capacity = 1;
                Self {
                    vec: Vec::with_capacity(capacity),
                    indexer: HexagonMapIndexer::new(r),
                }
            }
            HexWorldShape::Square(_, _) => todo!(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            vec: Vec::with_capacity(capacity),
            indexer: HexagonMapIndexer::new(1),
        }
    }

    pub fn set(&mut self, coord: HexCoord, value: V) -> Option<V> {
        let i = self.get_index(coord).unwrap();
        let p = self.vec[i];
        self.vec[i] = Some(value);

        p
    }

    pub fn get_index(&self, coord: HexCoord) -> Result<usize, TryFromIntError> {
        self.indexer.get(coord)
    }
}
