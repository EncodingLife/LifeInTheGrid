use bevy::{log::LogPlugin, prelude::*};
use bevy_pancam::{PanCam, PanCamPlugin};
use map::MapPlugin;
use sim_engine::SimWorld;
use simple_logger::SimpleLogger;

mod map;

#[derive(Resource, Debug)]
struct WorldWrapper(SimWorld);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(LogPlugin {
            filter: "info,wgpu_core=warn,wgpu_hal=warn,sim_interface=debug,apis=debug".into(),
            level: bevy::log::Level::DEBUG,
            ..Default::default()
        }), MapPlugin))
        .add_plugins((PanCamPlugin::default()))
        .insert_resource(WorldWrapper(sim_engine::build_world()))
        .add_systems(Startup, test)
        .run();
}

fn test(
    mut commands: Commands,sim_world: ResMut<WorldWrapper>) {
    commands.spawn(Camera2dBundle::default()).insert(PanCam::default());
}