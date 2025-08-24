use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// Import only what is used
use crate::camera::controller::{cursor_motion_system, spawn_camera};
use crate::input::cursor::{exit_on_esc, hide_cursor};
use crate::physics::config::setup_physics;
use crate::systems::player_movement::move_player;
use crate::world::setup::spawn_world;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(
            Startup,
            (spawn_world, spawn_camera, setup_physics, hide_cursor),
        )
        .add_systems(Update, (move_player, cursor_motion_system, exit_on_esc))
        .run();
}
