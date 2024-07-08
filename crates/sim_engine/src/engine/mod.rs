use apis::{FlatMap, HexWorld, HexWorldShape};
use settings::SimulationSettings;

use crate::{composition::CompoundComposition, compounds::Compound};


pub mod settings;

pub struct SimulationEngine {
    settings: SimulationSettings,
    element_map: FlatMap<CompoundComposition>
}

impl SimulationEngine {
    pub fn new(world_shape: HexWorldShape) -> Self {
        Self {
            settings: SimulationSettings::new(world_shape),
            element_map: FlatMap::init_with(world_shape, || CompoundComposition::default())
        }
    }

    pub fn world(&self) -> HexWorld<f32> {
        self.settings.world
    }

    pub fn element_map(&self) -> &FlatMap<CompoundComposition> {
        &self.element_map
    }
}