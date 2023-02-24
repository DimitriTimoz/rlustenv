use std::f32::consts::PI;

use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, DrawMesh2d}};
use bevy_rapier2d::{
    rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder},
    prelude::*,
};
use nalgebra::{inf, min, max};
use rlustenv::prelude::*;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_system(update_drone)
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
    configuration.gravity = Vec2::new(0.0, -9.81*0.01);
    // Add drone entity
    let drone_entity = commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(0.5, 0.25)))).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            ..default()
        })
        .insert(Drone {
            angle1: 0.0,
            angle2: 0.0,

            thrust1: 0.0,
            thrust2: 0.0,

        })
       // .insert(Controller::new("Drone"))
        .insert(AdditionalMassProperties::Mass(1.0))
        .insert(RigidBody::Dynamic)
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


}

fn update_drone(
    time: Res<Time>,
    mut drone_query: Query<(&mut Drone, &mut ExternalForce)>,
    mut right_prop_query: Query<(&mut RightPropulsion, &mut Transform, Without<LeftPropulsion>)>,
    mut left_prop_query: Query<(&mut LeftPropulsion, &mut Transform, Without<RightPropulsion>)>,
    keys: Res<Input<KeyCode>>,
) {
   // let (mut right_prop, mut right_transform) = right_prop_query.single_mut();
    let (mut left_prop, mut left_transform, () ) = left_prop_query.single_mut();
    let (mut right_prop, mut right_transform, () ) = right_prop_query.single_mut();

    for (mut drone, mut ext_force) in drone_query.iter_mut() {
        let mut dangle1 = 0.0;
        if keys.pressed(KeyCode::Q) {
            dangle1 += 1. * time.delta_seconds();
        }

        if keys.pressed(KeyCode::D) {
            dangle1 -= 1. * time.delta_seconds();
        }

        let mut dangle2 = 0.0;
        if keys.pressed(KeyCode::Left) {
            dangle2 += 1. * time.delta_seconds();
        }

        if keys.pressed(KeyCode::Right) {
            dangle2 -= 1. * time.delta_seconds();
        }


        drone.angle1 += dangle1;
        drone.angle2 += dangle2;
        const MAX_ANGLE: f32 = PI / 4.0;
        drone.angle1 = drone.angle1.min(MAX_ANGLE).max(-MAX_ANGLE);
        drone.angle2 = drone.angle2.min(MAX_ANGLE).max(-MAX_ANGLE);
        let mut thrust1 = 0.0;

        if keys.pressed(KeyCode::Z) {
            thrust1 = 1. * time.delta_seconds();
        } 
        let mut thrust2 = 0.0;
        if keys.pressed(KeyCode::Up) {
            thrust2 = 1. * time.delta_seconds();
        }
        
        let mut torque = 0.0;
        torque += thrust2 * drone.angle2.cos() - thrust1 * drone.angle1.cos();
        torque *= 0.0010;
        ext_force.torque = torque;
        
        left_transform.rotation = Quat::from_rotation_z(drone.angle1);
        right_transform.rotation = Quat::from_rotation_z(drone.angle2);
        // Compute Force
        let mut force = Vec2::new(0.0, 0.0);

        force += Vec2::new(thrust1 * drone.angle1.sin(), thrust1 * drone.angle1.cos());
        force += Vec2::new(thrust2 * drone.angle2.sin(), thrust2 * drone.angle2.cos());

        //ext_force.force = force;
        info!("torque: {}, force {}", torque, force);

    }
}


#[derive(Component, Default)]
struct Drone {
    thrust1: f32,
    angle1: f32,

    thrust2: f32,
    angle2: f32,
}

#[derive(Component, Default)]
struct RightPropulsion {
    angle: f32,
}

#[derive(Component, Default)]
struct LeftPropulsion {
    angle: f32,
}