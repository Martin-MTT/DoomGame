use bevy::math::primitives::{Cuboid, Plane3d};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    //Floor
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Plane3d::default())),
            material: materials.add(Color::rgb(0.2, 0.7, 0.3)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(10.0, 1.0, 10.0)),
            ..default()
        },
        Collider::cuboid(10.1, 0.1, 10.0),
    ));

    //Walls
    let wall = materials.add(Color::rgb(0.5, 0.2, 0.2));
    let wall_mesh = meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0)));

    let wall_positions = [
        (Vec3::new(0.0, 1.0, -5.0), Vec3::new(10.0, 2.0, 0.2)), //Front
        (Vec3::new(0.0, 1.0, 5.0), Vec3::new(10.0, 2.0, 0.2)),  //Back
        (Vec3::new(-5.0, 1.0, 0.0), Vec3::new(0.2, 2.0, 10.0)), //Left
        (Vec3::new(5.0, 1.0, 0.0), Vec3::new(0.2, 2.0, 10.0)),  //Right
    ];

    for (pos, scale) in wall_positions {
        commands.spawn((
            PbrBundle {
                mesh: wall_mesh.clone(),
                material: wall.clone(),
                transform: Transform {
                    translation: pos,
                    scale,
                    ..default()
                },
                ..default()
            },
            Collider::cuboid(scale.x / 2.0, scale.y / 2.0, scale.z / 2.0),
        ));
    }
}
