use apis::{HexWorld, HexWorldShape};

pub struct SimulationSettings {
    pub world: HexWorld<f32>
}

impl SimulationSettings {
    pub fn new(world_shape: HexWorldShape) -> Self {
        Self {
            world: HexWorld::new(apis::HexOrientation::Flat, 50.0, world_shape)
        }
    }
}