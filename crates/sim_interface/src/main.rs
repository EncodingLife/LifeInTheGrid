use bevy::{app::{App, Startup}, ecs::system::{ResMut, Resource}, DefaultPlugins, MinimalPlugins};
use sim_engine::SimWorld;

#[derive(Resource, Debug)]
struct WorldWrapper(SimWorld);

fn main() {
    println!("Main");
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WorldWrapper(sim_engine::build_world()))
        .add_systems(Startup, test)
        .run();
}

fn test(sim_world: ResMut<WorldWrapper>) {
    println!("sim_world: {:?}", sim_world);
}