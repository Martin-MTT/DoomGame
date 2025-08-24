use crate::components::player::Player;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &Transform), With<Camera>>,
) {
    for (mut velocity, transform) in &mut query {
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

        let speed = 5.0;
        velocity.linvel = direction.normalize_or_zero() * speed;
    }
}
