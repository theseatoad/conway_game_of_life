use std::fs;

use bevy::{prelude::*, render::texture::ImageSettings, time::FixedTimestep};
use cgol::{Tile, World};

//MAPWIDTH * TILEWIDTH < 700

//In tile units
pub const MAPWIDTH: u32 = 25;
pub const TILEWIDTH: u32 = 25;
pub const XOFFSET: u32 = 300;
pub const YOFFSET: u32 = 300;
pub const TICKRATE : f32 = 0.5;
struct CWorld {
    world: World,
}
impl Default for CWorld {
    fn default() -> Self {
        let mut tiles: Vec<Tile> = Vec::new();
        let world_string =
            fs::read_to_string("../assets/world.txt").expect("Could not read world file");
        for mut tile in world_string.split(",") {
            tile = tile.trim_matches('\n');
            match tile {
                "0" => {
                    tiles.push(Tile::new(false));
                }
                "1" => {
                    tiles.push(Tile::new(true));
                }
                _ => {
                    println!("Invalid tile");
                }
            }
        }
        CWorld {
            world: World { width: 25, tiles },
        }
    }
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
        .insert_resource(CWorld::default())
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TICKRATE as f64))
                .with_system(draw_world),
        )
        .run();
}

fn startup(mut commands: Commands) {
    //Spawn camera
    commands.spawn_bundle(Camera2dBundle::default());
}

fn draw_world(mut commands: Commands, asset_server: Res<AssetServer>, mut world: ResMut<CWorld>) {
    let mut x = 0;
    let mut y = 0;
    for tile in &world.world.tiles {
        match tile.on {
            true => {
                commands.spawn_bundle(SpriteBundle {
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
                });
            }
            false => {
                commands.spawn_bundle(SpriteBundle {
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
                });
            }
        }
        if x == MAPWIDTH - 1 {
            x = 0;
            y += 1;
        } else {
            x += 1
        }
    }
    world.world.tick();
}
