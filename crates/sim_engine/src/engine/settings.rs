use apis::{HexWorld, HexWorldShape};

pub struct SimulationSettings {
    pub world: HexWorld<f32>,
    pub cell_space: u16
}

impl SimulationSettings {
    pub fn new(world_shape: HexWorldShape) -> Self {
        Self {
            world: HexWorld::new(apis::HexOrientation::Flat, 20.0, world_shape),
            cell_space: 12
        }
    }
}