use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, DrawMesh2d}};
use bevy_rapier2d::{
    rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder},
    prelude::*,
};use rlustenv::prelude::*;

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
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(50.0, 25.0)))).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            ..default()
        })
        .insert(Drone {
            forward_thrust: 0.0,
            strafe_thrust: 0.0,
            angular_thrust: 0.0,
        })
        .insert(Controller::new("Drone"))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(25.0, 12.5))
        .insert(ExternalImpulse {
            impulse: Vec2::new(0.0, 1.0),
            torque_impulse: 2.0,
        })

        .id();

    // Add propulsion entities
    let propulsion_entity_left = commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(10.0, 10.0)))).into(),
            material: materials.add(ColorMaterial::from(Color::GREEN)),
            transform: Transform::from_xyz(-25.0, 0.0, 0.0),
            ..default()
        })
        .insert(Propulsion { force: 10.0 })
        .id();

    let propulsion_entity_right = commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(10.0, 10.0)))).into(),
            material: materials.add(ColorMaterial::from(Color::GREEN)),
            transform: Transform::from_xyz(25.0, 0.0, 0.0),
            ..default()
        })
        .insert(Propulsion { force: 10.0 })
        .id();

     // Attach propulsion entities to drone
     commands
        .entity(drone_entity)
        .push_children(&[propulsion_entity_right, propulsion_entity_left]);


}



#[derive(Component, Default)]
struct Drone {
    forward_thrust: f32,
    strafe_thrust: f32,
    angular_thrust: f32,
}

#[derive(Component, Default)]

struct Propulsion {
    force: f32,
}