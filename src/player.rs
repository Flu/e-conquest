use bevy::prelude::*;

use crate::{Laser, Materials, WinSize, Player, PlayerReadyFire, Speed, TIME_STEP};

// Create player plugin - responsible for all player related systems, entities and components
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_startup_stage("game-setup-actors", SystemStage::single(player_spawn.system()))
        .add_system(player_movement.system())
        .add_system(player_fire.system())
        .add_system(laser_movement.system());
    }
}

fn player_spawn(
    mut commands: Commands,
    win_size: Res<WinSize>,
    materials: Res<Materials>
    ){

    let bottom = -win_size.h/2.;
    // Spawn sprite
    commands.spawn_bundle(SpriteBundle {
        material: materials.player_materials.clone(),
        transform: Transform {
            translation: Vec3::new(0., bottom + 75./2. + 10., 10.),
            scale: Vec3::new(0.4,0.4,1.),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Player)
    .insert(PlayerReadyFire(true))
    .insert(Speed { horz: 200., vert: 200., ..Default::default()});
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Speed, &mut Transform, With<Player>)>
) {
    if let Ok((speed, mut transform, _)) = player_query.single_mut() {
        let mut direction: Vec2 = Vec2::default();
        direction.x = if keyboard_input.pressed(KeyCode::A) {
            -1.
        } else if keyboard_input.pressed(KeyCode::D) {
            1.
        } else {
            0.
        };

        direction.y = if keyboard_input.pressed(KeyCode::W) {
            1.
        } else if keyboard_input.pressed(KeyCode::S) {
            -1.
        } else {
            0.
        };

        let rotation = if keyboard_input.pressed(KeyCode::Q) {
            -1.
        } else if keyboard_input.pressed(KeyCode::E){
            1.
        } else {
            0.
        };

/*         let mut actual_speed = speed.horz;
        if transform.translation.x <= 50. && transform.translation.x >= win_size.w - 50. {
            actual_speed = 0.;
        } */

        transform.translation.x += direction.x * speed.horz * TIME_STEP;
        transform.translation.y += direction.y * speed.vert * TIME_STEP;
        transform.rotation *= Quat::from_rotation_z(rotation*speed.rot*TIME_STEP);
    }
}

fn player_fire(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    materials: Res<Materials>,
    mut player_query: Query<(&Transform, &mut PlayerReadyFire, With<Player>)>
) {
    if let Ok((player_transform, mut ready_fire, _)) = player_query.single_mut() {
        if ready_fire.0 && (keyboard_input.pressed(KeyCode::Space)
        || mouse_input.pressed(MouseButton::Left)) {
            let x = player_transform.translation.x;
            let y = player_transform.translation.y;

            //let mouse_x = ;

            commands.spawn_bundle(SpriteBundle {
                material: materials.laser_materials.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 0.),
                    scale: Vec3::new(0.01,0.01,1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Laser)
            .insert(Speed::default());
            ready_fire.0 = false;
        }

        if keyboard_input.just_released(KeyCode::Space) 
        || mouse_input.just_released(MouseButton::Left) {
            ready_fire.0 = true;
        }
    }
}

fn laser_movement(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Speed, &mut Transform, With<Laser>)>
) {
    for (laser_entity, speed, mut laser_transform, _) in query.iter_mut() {
        let translation = &mut laser_transform.translation;
        translation.y += speed.vert * TIME_STEP;
        if translation.y > win_size.h {
            commands.entity(laser_entity).despawn();
        }
    }
}