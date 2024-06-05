use shipyard::World;

#[derive(Debug)]
pub struct SimWorld(World);

pub fn build_world() -> SimWorld {
    SimWorld(World::new())
}