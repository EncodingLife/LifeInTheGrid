use std::mem::{self, size_of_val};

use sim_engine::{apis::{FlatMap, HexWorldShape}, composition::CompoundComposition};

fn main() {
    let world_shape = HexWorldShape::Hexagon(300);
    let flat_map = FlatMap::init_with(world_shape, || CompoundComposition::default());

    println!("Size of composition: {} kb", mem::size_of::<CompoundComposition>());

    println!("Size of FlatMap is {}", flat_map.footprint());

}
