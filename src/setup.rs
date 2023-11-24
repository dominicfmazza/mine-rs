use crate::player::{Player, PlayerCamera};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 50.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        PlayerCamera,
    ));
}

fn setup_cube(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    material: &Handle<StandardMaterial>,
) {
    let cube_size = Vec3::new(0.5, 0.5, 0.5);
    commands
        .spawn(RigidBody::Dynamic)
        .insert((
            Collider::cuboid(cube_size.x, cube_size.y, cube_size.z),
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube {
                    size: cube_size.x * 2.,
                })),
                material: material.clone(),
                ..default()
            },
            Player,
        ))
        .insert(KinematicCharacterController {
            snap_to_ground: Some(CharacterLength::Absolute(0.5)),
            ..default()
        })
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
}

fn setup_ground(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    material: &Handle<StandardMaterial>,
) {
    let ground_size = Vec3::new(50., 0.1, 50.);

    commands
        .spawn((
            Collider::cuboid(ground_size.x, ground_size.y, ground_size.z),
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box {
                    min_x: -ground_size.x,
                    max_x: ground_size.x,
                    min_y: -ground_size.y,
                    max_y: ground_size.y,
                    min_z: -ground_size.z,
                    max_z: ground_size.z,
                })),
                material: material.clone(),
                ..default()
            },
        ))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));
}

fn setup_light(commands: &mut Commands) {
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

pub fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    server: Res<AssetServer>,
) {
    let ground_handle: Handle<Image> = server.load("textures/debug_ground.png");
    let ground_material = materials.add(StandardMaterial {
        base_color_texture: Some(ground_handle),
        ..default()
    });
    let cube_handle: Handle<Image> = server.load("textures/debug_square.png");
    let cube_material = materials.add(StandardMaterial {
        base_color_texture: Some(cube_handle),
        ..default()
    });

    setup_ground(&mut commands, &mut meshes, &ground_material);
    setup_cube(&mut commands, &mut meshes, &cube_material);
    setup_light(&mut commands);
}
