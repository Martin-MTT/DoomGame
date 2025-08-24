use crate::components::player::Player;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &Transform, &Player)>,
) {
    for (mut velocity, transform, player) in &mut query {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += *transform.forward();
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction -= *transform.forward();
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction -= *transform.right();
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += *transform.right();
        }

        let horizontal_velocity = direction.normalize_or_zero() * player.speed;

        // Keep vertical velocity (Y) from physics (gravity, jumping, etc.)
        velocity.linvel = Vec3::new(
            horizontal_velocity.x,
            velocity.linvel.y, // Preserve current Y velocity
            horizontal_velocity.z,
        );
    }
}
