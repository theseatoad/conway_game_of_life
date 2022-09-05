use std::fs;

use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_inspector_egui::WorldInspectorPlugin;
use cgol::Tile;

//MAPWIDTH * TILEWIDTH < 700

//In tile units
pub const MAPWIDTH: u32 = 25;
pub const TILEWIDTH: u32 = 25;
pub const XOFFSET: u32 = 300;
pub const YOFFSET: u32 = 300;

#[derive(Component)]
struct Position {
    x: u32,
    y: u32,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Game of life".to_string(),
            width: 700.,
            height: 800.,
            present_mode: bevy::window::PresentMode::Fifo,
            ..default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(startup)
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {

    //Spawn camera
    commands.spawn_bundle(Camera2dBundle::default());

    let mut tiles : Vec<Tile> = Vec::new();

    //Read tiles from file.
    let world_string = fs::read_to_string("assets/world.txt").expect("Could not read world file");
    let mut x = 0;
    let mut y = 0;
    for mut tile in world_string.split(","){
        tile = tile.trim_matches('\n');
        if !tile.is_empty() {
            match tile {
                "0" => {
                    commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::WHITE.into(),
                            ..default()
                        },
                        texture: asset_server.load("tile.png"),
                        transform: Transform {
                            translation: Vec3 {
                                x: ((x * TILEWIDTH) as f32) - XOFFSET as f32,
                                y: ((y * TILEWIDTH) as f32) - YOFFSET as f32,
                                z: 1.,
                            },
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Position { x, y });

                    tiles.push(Tile::new(true));
                }
                "1" => {
                    commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::BLACK.into(),
                            ..default()
                        },
                        texture: asset_server.load("tile.png"),
                        transform: Transform {
                            translation: Vec3 {
                                x: ((x * TILEWIDTH) as f32) - XOFFSET as f32,
                                y: ((y * TILEWIDTH) as f32) - YOFFSET as f32,
                                z: 1.,
                            },
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Position { x, y });

                    tiles.push(Tile::new(true));

                }
                _ => {
                    println!("Non-valid tile.")
                }
            }
        }
        if x == MAPWIDTH - 1 {
            x = 0;
            y += 1;
        } else {
            x += 1
        }
    }
    
}
