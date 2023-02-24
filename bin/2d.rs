use std::f32::consts::PI;

use bevy::{prelude::*, sprite::{MaterialMesh2dBundle}};
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier2d::{
    rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder},
    prelude::*,
};
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
    configuration.gravity = Vec2::new(0.0, -9.81);
    // Add drone entity
    let drone_entity = commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(0.5, 0.25)))).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            ..default()
        })
        .insert(Drone)
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
    mut drone_query: Query<(&mut ExternalForce, &Transform, With<Drone>, Without<LeftPropulsion>, Without<RightPropulsion>)>,
    mut right_prop_query: Query<(&mut RightPropulsion, &mut Transform, Without<LeftPropulsion>)>,
    mut left_prop_query: Query<(&mut LeftPropulsion, &mut Transform, Without<RightPropulsion>)>,
    mut lines: ResMut<DebugLines>,
    keys: Res<Input<KeyCode>>,
) {
   // let (mut right_prop, mut right_transform) = right_prop_query.single_mut();
    let (mut left_prop, mut left_transform, () ) = left_prop_query.single_mut();
    let (mut right_prop, mut right_transform, () ) = right_prop_query.single_mut();

    for (mut ext_force, trans, _, _, _) in drone_query.iter_mut() {
        let angle = -trans.rotation.z * PI;
        info!("Angle: {}", angle);
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


        left_prop.angle += dangle1;
        right_prop.angle += dangle2;
        const MAX_ANGLE: f32 = PI / 4.0;
        left_prop.angle = left_prop.angle.min(MAX_ANGLE).max(-MAX_ANGLE);
        right_prop.angle = right_prop.angle.min(MAX_ANGLE).max(-MAX_ANGLE);
        let mut thrust1 = 0.0;

        if keys.pressed(KeyCode::Z) {
            thrust1 = 1.;
        } 
        let mut thrust2 = 0.0;
        if keys.pressed(KeyCode::Up) {
            thrust2 = 1.;
        }
        
        let mut torque = 0.0;
        torque += thrust2 * right_prop.angle.cos() - thrust1 * left_prop.angle.cos();
        torque *= 0.0000050;
        ext_force.torque = torque;
        
        left_transform.rotation = Quat::from_rotation_z(left_prop.angle);
        right_transform.rotation = Quat::from_rotation_z(right_prop.angle);
        // Compute Force
        let mut force = Vec2::new(0.0, 0.0);

        force += Vec2::new(thrust1 * ( angle).sin(), thrust1 * ( angle).cos());
        force += Vec2::new(thrust2 * ( angle).sin(), thrust2 * (  angle).cos());

        ext_force.force = force * 15.;
        info!("torque: {}, force {}", torque, force);

        lines.line(trans.translation, trans.translation + Vec3 { x: ext_force.force.x, y: ext_force.force.y, z: 0. }, time.delta_seconds());
    }
}


#[derive(Component)]
struct Drone;

#[derive(Component, Default)]
struct RightPropulsion {
    angle: f32,
    thrust: f32,
}

#[derive(Component, Default)]
struct LeftPropulsion {
    angle: f32,
    thrust: f32,
}