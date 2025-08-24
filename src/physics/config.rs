use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn setup_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec3::new(0.0, -10.0, 0.0);
}
