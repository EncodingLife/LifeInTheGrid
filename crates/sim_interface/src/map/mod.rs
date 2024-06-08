use apis::{HexCoord, HexCoordinate, HexLayout, Hexagon};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, draw_map);
    }
}

fn draw_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let layout = HexLayout::new(60.0);
    let hexagon_mesh_handle = meshes.add(Hexagon::new(layout));

    commands.spawn(MaterialMesh2dBundle {
        mesh: hexagon_mesh_handle.clone().into(),
        transform: Transform::from_translation(bevy::prelude::Vec3::new(0.0, 0., 0.)),
        material: materials.add(ColorMaterial::from(Color::GREEN)),
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle {
        mesh: hexagon_mesh_handle.clone().into(),
        transform: Transform::from_translation(HexCoord::new(1,0,-1).to_world_v3(layout)),
        material: materials.add(ColorMaterial::from(Color::RED)),
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle {
        mesh: hexagon_mesh_handle.clone().into(),
        transform: Transform::from_translation(HexCoord::new(1,1,-2).to_world_v3(layout)),
        material: materials.add(ColorMaterial::from(Color::ORANGE)),
        ..default()
    });
}