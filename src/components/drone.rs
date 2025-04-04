use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::prelude::*;

#[derive(Clone)]
pub enum DroneEndReason {
    OutOfBounds,
    Dead,
    Crashed,
    ReachedTarget,
}

impl DroneEndReason {
    pub fn to_string(&self) -> String {
        match self {
            DroneEndReason::OutOfBounds => "out_of_bounds".to_string(),
            DroneEndReason::Dead => "dead".to_string(),
            DroneEndReason::Crashed => "crashed".to_string(),
            DroneEndReason::ReachedTarget => "reached_target".to_string(),
        }
    }
}

#[derive(Bundle)]
pub struct DroneBundle {
    pub controller: DroneController,
    pub mass: AdditionalMassProperties,
    rigid_body: RigidBody,
    velocity: Velocity,
    drone: Drone,
}

#[derive(Component)]
pub struct Drone;

impl Default for DroneBundle {
    fn default() -> Self {
        Self {
            controller: DroneController::new("Drone"),
            mass: AdditionalMassProperties::Mass(1.0),
            rigid_body: RigidBody::default(),
            velocity: Velocity::default(),
            drone: Drone,
        }
    }
}

impl DroneBundle {
    pub fn do_spawn_drone(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>) {
        // Add drone entity to a random position

        let mut rng = rand::thread_rng();
        let random_vec = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)) * 10.0;

        let drone_entity = commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Quad::new(Vec2::new(0.5, 0.25))))
                    .into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                ..default()
            })
            .insert(DroneBundle::default())
            .insert(Collider::cuboid(0.25, 0.125))
            .insert(TransformBundle::from(Transform::from_xyz(random_vec.x, random_vec.y, 0.0)))
            .insert(ExternalForce {
                force: Vec2::new(0.0, 0.0),
                torque: 0.0,
            })
            .id();

        // Add propulsion entities
        let propulsion_entity_left = commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Quad::new(Vec2::new(0.10, 0.2))))
                    .into(),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                transform: Transform::from_xyz(-0.250, 0.1, 1.0),
                ..default()
            })
            .insert(LeftPropulsion::default())
            .id();

        let propulsion_entity_right = commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Quad::new(Vec2::new(0.1, 0.2))))
                    .into(),
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
}

#[derive(Component, Default)]
pub struct RightPropulsion {
    angle: f32,
    thrust: f32,
}

#[derive(Component, Default)]
pub struct LeftPropulsion {
    angle: f32,
    thrust: f32,
}
type DroneQuery<'a> = (
    &'a mut ExternalForce,
    &'a Transform,
    &'a Velocity,
    With<Drone>,
    Without<LeftPropulsion>,
    Without<RightPropulsion>,
);

impl DroneBundle {
    pub fn update_drone_inputs(
        keys: Res<Input<KeyCode>>,
        time: Res<Time>,
        mut right_prop_query: Query<&mut RightPropulsion>,
        mut left_prop_query: Query<&mut LeftPropulsion>,
    ) {
        if left_prop_query.iter().count() == 0 {
            return;
        }

        if right_prop_query.iter().count() == 0 {
            return;
        }

        let mut left_prop = left_prop_query.single_mut();
        let mut right_prop = right_prop_query.single_mut();

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

        if keys.pressed(KeyCode::Z) {
            left_prop.thrust += 0.5 * time.delta_seconds();
        } else if keys.pressed(KeyCode::S) {
            left_prop.thrust -= 0.5 * time.delta_seconds();
        }

        left_prop.thrust = left_prop.thrust.clamp(0.0, 1.0);

        if keys.pressed(KeyCode::Up) {
            right_prop.thrust += 0.5 * time.delta_seconds();
        } else if keys.pressed(KeyCode::Down) {
            right_prop.thrust -= 0.5 * time.delta_seconds();
        }

        left_prop.angle += dangle1;
        right_prop.angle += dangle2;
    }

    pub fn update_drone(
        target_query: Query<(
            &Transform,
            With<Target>,
            Without<Drone>,
            Without<LeftPropulsion>,
            Without<RightPropulsion>,
        )>,
        mut drone_query: Query<DroneQuery>,
        mut right_prop_query: Query<(
            &mut RightPropulsion,
            &mut Transform,
            Without<LeftPropulsion>,
        )>,
        mut left_prop_query: Query<(
            &mut LeftPropulsion,
            &mut Transform,
            Without<RightPropulsion>,
        )>,
        mut drone_controller_query: Query<&mut DroneController>,
        mut lines: ResMut<DebugLines>,
    ) {
        if left_prop_query.iter().count() == 0 {
            return;
        }

        if right_prop_query.iter().count() == 0 {
            return;
        }

        let target = target_query.single().0;
        let (mut left_prop, mut left_transform, ()) = left_prop_query.single_mut();
        let (mut right_prop, mut right_transform, ()) = right_prop_query.single_mut();

        for (mut ext_force, trans, velocity, _, _, _) in drone_query.iter_mut() {
            let angle = -trans.rotation.z * PI;

            // Get the inputs given by the controller
            let drone_controller = drone_controller_query.single();
            let (left_thrust, right_thrust) = drone_controller.get_thrust();
            let (left_angle, right_angle) = drone_controller.get_thrust_angle();

            // Update the propellers
            left_prop.angle = left_angle;
            right_prop.angle = right_angle;
            left_prop.thrust = left_thrust;
            right_prop.thrust = right_thrust;

            const MAX_ANGLE: f32 = PI / 3.0;
            left_prop.angle = left_prop.angle.clamp(-MAX_ANGLE, MAX_ANGLE);
            right_prop.angle = right_prop.angle.clamp(-MAX_ANGLE, MAX_ANGLE);

            right_prop.thrust = right_prop.thrust.clamp(0.0, 1.0);

            let mut torque = 0.0;
            torque += right_prop.thrust * right_prop.angle.cos()
                - left_prop.thrust * left_prop.angle.cos();
            torque *= 0.0000050;
            ext_force.torque = torque;

            left_transform.rotation = Quat::from_rotation_z(left_prop.angle);
            right_transform.rotation = Quat::from_rotation_z(right_prop.angle);

            // Compute Force
            let mut force = Vec2::new(0.0, 0.0);

            force += Vec2::new(
                left_prop.thrust * (angle - left_prop.angle).sin(),
                left_prop.thrust * (angle - left_prop.angle).cos(),
            );
            force += Vec2::new(
                right_prop.thrust * (angle - right_prop.angle).sin(),
                right_prop.thrust * (angle - right_prop.angle).cos(),
            );

            ext_force.force = force * 15.;

            // Update drone controller
            drone_controller_query.single_mut().update_properties(
                trans,
                velocity.linvel.into(),
                velocity.angvel,
                (target.translation - trans.translation).truncate().into(),
            );

            lines.line_colored(
                trans.translation,
                trans.translation
                    + Vec3 {
                        x: ext_force.force.x,
                        y: ext_force.force.y,
                        z: 0.,
                    } * 0.1,
                0.0,
                Color::RED,
            );
            lines.line_colored(
                trans.translation,
                trans.translation
                    + Vec3 {
                        x: velocity.linvel.x,
                        y: velocity.linvel.y,
                        z: 0.0,
                    },
                0.0,
                Color::BLUE,
            );
        }
    }

    pub fn check_end(
        mut commands: Commands,
        mut drone_query: Query<(
            Entity,
            &Transform,
            &Velocity,
            &mut DroneController,
            Without<LeftPropulsion>,
        )>,
        // Get the target
        target_query: Query<(&Transform, With<Target>)>,

        // To spawn a new drone
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let target = target_query.single().0;
        for (entity, transfrom, _velocity, mut drone_controller, _) in drone_query.iter_mut() {
            let mut end = None;

      
            if transfrom.rotation.z.abs() > 0.6 {
                end = Some(DroneEndReason::Crashed);
            }

            if (target.translation - transfrom.translation).length() < 0.1 {
                end = Some(DroneEndReason::ReachedTarget);
            } else if (target.translation - transfrom.translation).length() > 200. {
                end = Some(DroneEndReason::OutOfBounds);
            }

            if let Some(reason) = end {
                match drone_controller.end(reason.clone()) {
                    Ok(_) => {}
                    Err(e) => {
                        error!("Error while ending the drone: {}", e);
                    }
                }
                commands.entity(entity).despawn_descendants();
                commands.entity(entity).despawn();

                DroneBundle::do_spawn_drone(&mut commands, &mut meshes, &mut materials);
            }
        }
    }

    /// Detect if the drone failed
    pub fn get_system_set() -> SystemSet {
        SystemSet::new()
            .with_system(Self::check_end)
            .with_system(Self::update_drone_inputs)
            .with_system(Self::update_drone)
    }
}
