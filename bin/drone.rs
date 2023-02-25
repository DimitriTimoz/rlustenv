
use bevy::{prelude::*, sprite::{MaterialMesh2dBundle}};
use bevy_rapier2d::prelude::*;
use rlustenv::prelude::*;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_plugin(RlustenvPlugin)
        .run();
}


fn setup(
    mut commands: Commands,
    mut configuration: ResMut<RapierConfiguration>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Configure Bevy_Rapier physics
    configuration.gravity = Vec2::new(0.0, -9.81);
    // Add drone entity
    let drone_entity = commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(0.5, 0.25)))).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            ..default()
        })
        .insert(DroneBundle::default())
        .insert(Collider::cuboid(0.25, 0.125))
        .insert(ExternalForce {
            force: Vec2::new(0.0, 0.0),
            torque: 0.0,
        })

        .id();

    // Add propulsion entities
    let propulsion_entity_left = commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(0.10, 0.2)))).into(),
            material: materials.add(ColorMaterial::from(Color::GREEN)),
            transform: Transform::from_xyz(-0.250, 0.1, 1.0),
            ..default()
        })
        .insert(LeftPropulsion::default())
        .id();

    let propulsion_entity_right = commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(0.1, 0.2)))).into(),
            material: materials.add(ColorMaterial::from(Color::GREEN)),
            transform: Transform::from_xyz(0.250, 0.1, 1.0),
            ..default()
        })
        .insert(RightPropulsion::default())
        .id();

     // Attach propulsion entities to drone
     commands
        .entity(drone_entity)
        .push_children(&[propulsion_entity_right, propulsion_entity_left]);

    // Add ground entity
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(100.0, 1.0)))).into(),
            material: materials.add(ColorMaterial::from(Color::BLACK)),
            transform: Transform::from_xyz(0.0, -5.0, 0.0),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(50.0, 0.5));


}



