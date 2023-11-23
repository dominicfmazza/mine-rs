use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_rapier3d::prelude::*;
pub mod setup;


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .add_systems(Startup, (setup::setup_graphics, setup::setup_physics))
        .add_systems(Update, print_ball_altitude)
        // .add_systems(Update, (update_player_position, handle_player_input))
        .run();
}

#[derive(Component)]
struct Player {
    movement_speed: f32,
    velocity: Vec2,
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}


// fn handle_player_input(mut query: Query<&mut Player>, keyboard: Res<Input<KeyCode>>) {
//     for (mut player) in query.iter_mut() {
//         if keyboard.pressed(KeyCode::W) {
//             player.velocity.x += player.movement_speed;
//         }
//         if keyboard.pressed(KeyCode::A) {
//             player.velocity.y -= player.movement_speed;
//         }
//         if keyboard.pressed(KeyCode::S) {
//             player.velocity.x -= player.movement_speed;
//         }
//         if keyboard.pressed(KeyCode::D) {
//             player.velocity.y += player.movement_speed;
//         }
//     }
// }
//
// fn update_player_position(mut query: Query<(&mut KinematicCharacterController, &Player)>) {
//     for (mut controller, player) in query.iter_mut() {
//         controller.custom_shape = player.velocity;
//     }
// }
