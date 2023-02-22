use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use rlustenv::prelude::*;

fn main() {
    rlustenv::app::App::new()
        .add_startup_system(setup_physics)
        .run();
}


fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(Controller::default())
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)))
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Box::new(1000.0, 100.0, 0.0)))
                .into(),
            transform: Transform::default(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            ..default()
        });

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.7))
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Circle {
                    radius: 50.0,
                    ..Default::default()
                }))
                .into(),
            transform: Transform::default().with_scale(Vec3::splat(128.)),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
}

