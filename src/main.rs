use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .add_systems(Startup, (setup_graphics, setup_physics))
        .add_systems(Update, print_ball_altitude)
        // .add_systems(Update, (update_player_position, handle_player_input))
        .run();
}

#[derive(Component)]
struct Player {
    movement_speed: f32,
    velocity: Vec2,
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(20.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(50.0, 0.1, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert((
            Collider::cuboid(0.5, 0.5, 0.5),
            PbrBundle {
                mesh: meshes.add(shape::Cube::default().into()),
                material: debug_material.clone(),
                ..default()
            },
        ))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
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
