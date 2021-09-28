#![allow(unused)]

// Introduce the player module
mod player;

// Import elements from modules
use bevy::prelude::*;
use player::PlayerPlugin;

// region: Constants

const PLAYER_SPRITE: &str = "player_a_01.png";
const LASER_SPRITE: &str = "laser_sprite.png";
const TIME_STEP: f32 = 1./60.;

// endregion: Constants

// region: Resources

pub struct Materials {
    player_materials: Handle<ColorMaterial>,
    laser_materials: Handle<ColorMaterial>,
}

struct WinSize {
    w: f32,
    h: f32,
}
// endregion: Resources

// region: Components

struct Player;
struct PlayerReadyFire(bool);
struct Laser;
struct Speed(f32);

impl Default for Speed {
    fn default() -> Self {
        Self(500.)
    }
}

// endregion: Components

fn main() {
    App::build()
    .insert_resource(ClearColor(Color::rgb(0.04,0.04,0.04)))
    .insert_resource(WindowDescriptor {
        title: "Little game".to_string(),
        width: 600.0,
        height: 600.0,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(PlayerPlugin)
    .add_startup_system(setup.system())
    .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>
    ){

    // Build material resources for sprites
    commands.insert_resource(Materials {
        player_materials: materials.add(asset_server.load(PLAYER_SPRITE).into()),
        laser_materials: materials.add(asset_server.load(LASER_SPRITE).into()),
    });

    // Build resource for window information
    let mut window = windows.get_primary_mut().unwrap();
    commands.insert_resource(WinSize {
        w: window.width(),
        h: window.height(),
    });

    // Spawn 2D camera bundle, it represents the point of view of the player
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}