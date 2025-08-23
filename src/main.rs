use bevy::app::AppExit;
use bevy::input::mouse::MouseMotion;
use bevy::math::primitives::Cuboid;
use bevy::math::primitives::Plane3d;
use bevy::prelude::EventReader;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (setup, hide_cursor, setup_physics))
        .add_systems(Update, (move_player, cursor_motion_system, exit_on_esc))
        .run();
}

#[derive(Component)]
struct CameraController {
    yaw: f32,
    pitch: f32,
    sensitivity: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 1.5, 5.0), // Position the camera
                rotation: Quat::from_rotation_y(0.0),  // Facing forward (optional)
                scale: Vec3::ONE,                      // Default scale (1,1,1)
            },
            ..default()
        },
        CameraController {
            yaw: 0.0,
            pitch: 0.0,
            sensitivity: 0.001,
        },
        RigidBody::Dynamic,
        Collider::capsule_y(0.75, 0.3), // approximate player collider
        Velocity::default(),
        LockedAxes::ROTATION_LOCKED, // prevent player from tipping over
    ));

    // Floor
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Plane3d::default())),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.2, 0.7, 0.3),
                ..default()
            }),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0), // Position the floor at origin
                rotation: Quat::IDENTITY,              // No rotation
                scale: Vec3::new(10.0, 1.0, 10.0),     // Scale floor to 10x10 units
            },
            ..default()
        },
        Collider::cuboid(10.1, 0.1, 10.0),
    ));

    // Walls (4 walls surrounding the floor)
    let wall_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.5, 0.2, 0.2),
        ..default()
    });

    // Front wall
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0))),
            material: wall_material.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 1.0, -5.0), // Z is negative
                scale: Vec3::new(10.0, 2.0, 0.2),       // Wide and flat
                ..default()
            },
            ..default()
        },
        Collider::cuboid(10.1, 0.1, 10.0),
    ));

    // Back wall
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0))),
            material: wall_material.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 1.0, 5.0),
                scale: Vec3::new(10.0, 2.0, 0.2),
                ..default()
            },
            ..default()
        },
        Collider::cuboid(10.1, 0.1, 10.0),
    ));

    // Left wall
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0))),
            material: wall_material.clone(),
            transform: Transform {
                translation: Vec3::new(-5.0, 1.0, 0.0),
                scale: Vec3::new(0.2, 2.0, 10.0),
                ..default()
            },
            ..default()
        },
        Collider::cuboid(10.1, 0.1, 10.0),
    ));

    // Right wall
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0))),
            material: wall_material,
            transform: Transform {
                translation: Vec3::new(5.0, 1.0, 0.0),
                scale: Vec3::new(0.2, 2.0, 10.0),
                ..default()
            },
            ..default()
        },
        Collider::cuboid(10.1, 0.1, 10.0),
    ));
}

fn setup_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec3::new(0.0, -10.0, 0.0); // Gravity pulls downwards on Y-axis
}

fn move_player(
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

fn cursor_motion_system(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut CameraController)>,
) {
    let (mut transform, mut controller) = match query.get_single_mut() {
        Ok(data) => data,
        Err(_) => return,
    };

    for event in mouse_motion_events.read() {
        controller.yaw -= event.delta.x * controller.sensitivity;
        controller.pitch -= event.delta.y * controller.sensitivity;

        controller.pitch = controller.pitch.clamp(-1.54, 1.54);

        let yaw_rot = Quat::from_rotation_y(controller.yaw);
        let pitch_rot = Quat::from_rotation_x(controller.pitch);
        transform.rotation = yaw_rot * pitch_rot;
    }
}

fn hide_cursor(mut q_window: Query<&mut Window>) {
    if let Ok(mut window) = q_window.get_single_mut() {
        window.cursor.visible = false;
        window.cursor.grab_mode = bevy::window::CursorGrabMode::Locked;
    }
}

fn exit_on_esc(keyboard_input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

//For debug, get current location
fn print_camera_position(query: Query<&Transform, With<Camera>>) {
    if let Ok(transform) = query.get_single() {
        println!("Camera position: {:?}", transform.translation);
    }
}
