use bevy::prelude::*;
use map::MapPlugin;
use sim_engine::SimWorld;

mod map;

#[derive(Resource, Debug)]
struct WorldWrapper(SimWorld);

fn main() {
    println!("Main");
    App::new()
        .add_plugins((DefaultPlugins, MapPlugin))
        .insert_resource(WorldWrapper(sim_engine::build_world()))
        .add_systems(Startup, test)
        .run();
}

fn test(
    mut commands: Commands,sim_world: ResMut<WorldWrapper>) {
    commands.spawn(Camera2dBundle::default());
    println!("sim_world: {:?}", sim_world);
}