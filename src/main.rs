// Import modules
mod enemy;
mod player;

// Import elements from modules
use bevy::{prelude::*, sprite::collide_aabb::collide};
use player::PlayerPlugin;
use enemy::EnemyPlugin;

// region: Constants

const PLAYER_SPRITE: &str = "player_a_01.png";
const LASER_SPRITE: &str = "laser_sprite.png";
const ENEMY_SPRITE: &str = "enemy_sprite.png";
const TIME_STEP: f32 = 1./60.;

// endregion: Constants

// region: Resources

pub struct Materials {
    player_materials: Handle<ColorMaterial>,
    laser_materials: Handle<ColorMaterial>,
    enemy_materials: Handle<ColorMaterial>
}

struct ActiveEnemies(u32);

struct WinSize {
    w: f32,
    h: f32,
}
// endregion: Resources

// region: Components

struct Player;
struct PlayerReadyFire(bool);
struct Enemy;
struct Laser;
struct Speed {
    vert: f32,
    horz: f32,
    rot: f32
}

impl Default for Speed {
    fn default() -> Self {
        Speed {
            vert: 500.,
            horz: 500.,
            rot: 1.
        }
    }
}

// endregion: Components

fn main() {
    App::build()
    .insert_resource(ClearColor(Color::rgb(0.04,0.04,0.04)))
    .insert_resource(WindowDescriptor {
        title: "Little game".to_string(),
        width: 300.0,
        height: 600.0,
        ..Default::default()
    })
    .insert_resource(ActiveEnemies(0))
    .add_plugins(DefaultPlugins)
    .add_plugin(PlayerPlugin)
    .add_system(laser_enemy_collision.system())
    .add_plugin(EnemyPlugin)
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
        enemy_materials: materials.add(asset_server.load(ENEMY_SPRITE).into())
    });

    // Build resource for window information
    let window = windows.get_primary_mut().unwrap();
    commands.insert_resource(WinSize {
        w: window.width(),
        h: window.height(),
    });

    // Spawn 2D camera bundle, it represents the point of view of the player
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn laser_enemy_collision(
    mut commands: Commands,
    mut laser_query: Query<(Entity, &Transform, &Sprite, With<Laser>)>,
    mut enemy_query: Query<(Entity, &Transform, &Sprite, With<Enemy>)>,
    mut active_enemies: ResMut<ActiveEnemies>
) {
    for (laser_entity, laser_tf, laser_sprite, _) in laser_query.iter_mut() {
        for (enemy_entity, enemy_tf, enemy_sprite, _) in enemy_query.iter_mut() {
            let laser_scale = Vec2::from(laser_tf.scale);
            let enemy_scale = Vec2::from(enemy_tf.scale);

            let collision = collide(
                laser_tf.translation, laser_sprite.size * laser_scale,
                enemy_tf.translation, enemy_sprite.size * enemy_scale
            );
            if let Some(_) = collision {
                commands.entity(enemy_entity).despawn();
                commands.entity(laser_entity).despawn();
                active_enemies.0 -= 1;
            }
        }
    }
}