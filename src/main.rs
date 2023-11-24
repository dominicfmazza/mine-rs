use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_rapier3d::prelude::*;
pub mod player;
pub mod setup;
pub mod update;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .add_systems(Startup, (setup::setup_graphics, setup::setup_physics))
        .add_systems(
            Update,
            (
                update::handle_player_input,
                update::follow_player_with_camera,
            ),
        )
        .run();
}
