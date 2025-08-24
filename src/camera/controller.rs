use crate::components::player::Player;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct CameraController {
    pub yaw: f32,
    pub pitch: f32,
    pub sensitivity: f32,
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.5, 5.0),
            ..default()
        },
        CameraController {
            yaw: 0.0,
            pitch: 0.0,
            sensitivity: 0.001,
        },
        Player::default(),
        RigidBody::Dynamic,
        Collider::capsule_y(0.75, 0.3),
        Velocity::default(),
        LockedAxes::ROTATION_LOCKED,
    ));
}

pub fn cursor_motion_system(
    mut motion: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut CameraController)>,
) {
    if let Ok((mut transform, mut controller)) = query.get_single_mut() {
        for ev in motion.read() {
            controller.yaw -= ev.delta.x * controller.sensitivity;
            controller.pitch -= ev.delta.y * controller.sensitivity;
            controller.pitch = controller.pitch.clamp(-1.54, 1.54);

            transform.rotation =
                Quat::from_rotation_y(controller.yaw) * Quat::from_rotation_x(controller.pitch);
        }
    }
}
