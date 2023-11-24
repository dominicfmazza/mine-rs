use crate::player::{Player, PlayerCamera};
use bevy::prelude::*;
use bevy_rapier3d::prelude::KinematicCharacterController;

pub fn handle_player_input(
    mut query: Query<&mut KinematicCharacterController, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
) {
    let mut controller = query.single_mut();
    let mut potential_translation = Vec3::new(0.0, 0.0, 0.0);
    if keyboard.pressed(KeyCode::W) {
        potential_translation.x -= 0.1;
    }
    if keyboard.pressed(KeyCode::A) {
        potential_translation.z += 0.1;
    }
    if keyboard.pressed(KeyCode::S) {
        potential_translation.x += 0.1;
    }
    if keyboard.pressed(KeyCode::D) {
        potential_translation.z -= 0.1;
    }
    controller.translation = Some(potential_translation);
}

pub fn follow_player_with_camera(
    mut camera_query: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let mut transformation = camera_query.single_mut();

    transformation.translation = player_query.single().translation;
    transformation.translation.y += 50.;
}
