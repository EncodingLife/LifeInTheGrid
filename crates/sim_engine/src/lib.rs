use shipyard::World;

mod elements;
mod engine;

pub use apis;
pub use crate::engine::*;
pub use crate::elements::*;

#[derive(Debug)]
pub struct SimWorld(World);

pub fn build_world() -> SimWorld {
    SimWorld(World::new())
}