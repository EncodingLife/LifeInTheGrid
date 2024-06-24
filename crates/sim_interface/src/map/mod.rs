use apis::{CellBucket, FlatMap, HexCoord, HexCoordinate, HexOrientation, HexWorld, HexWorldShape, Hexagon};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use sim_engine::{settings::SimulationSettings, ElementalComposition, SimulationEngine};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, draw_map);
    }
}

fn draw_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>) {
        let world_shape = HexWorldShape::Hexagon(20);
        let engine = SimulationEngine::new(world_shape);

        let cell_size = 50.0;

        let smaller_shape = Hexagon::new(HexOrientation::Flat, cell_size-1.0);
        let hexagon_mesh_handle = meshes.add(smaller_shape);
        debug!("World capacity: {}", engine.element_map().index().capacity());
        for bucket in engine.element_map().iter() {
            match bucket {
                CellBucket::Occupied(coord, value) => {
                    let translation = engine.world().coord_to_world_v3(*coord);
                    commands.spawn(Text2dBundle {
                        text: Text::from_section(
                                    format!("{}", engine.element_map().index().index(*coord)),
                                    TextStyle {
                                        font_size: 16.0,
                                        color: Color::WHITE,
                                        // if you want to use your game's font asset,
                                        // uncomment this and provide the handle:
                                        // font: my_font_handle
                                        ..default()
                                    }) ,
                                    transform: Transform::from_translation(Vec3{ x: translation.x, y: translation.y, z: 1.0 }),
                        ..Default::default()
                    });
                    commands.spawn(MaterialMesh2dBundle {
                        mesh: hexagon_mesh_handle.clone().into(),
                        transform: Transform::from_translation(translation),
                        material: materials.add(ColorMaterial::from(Color::NAVY)),
                        ..default()
                    });
                },
                CellBucket::Empty => {},
            }
        }

    }