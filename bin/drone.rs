
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
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Configure Bevy_Rapier physics
    configuration.gravity = Vec2::new(0.0, -9.81);

    // Add Drone entity
    DroneBundle::do_spawn_drone(&mut commands, &mut meshes, &mut materials);
    // Add ground entity
    /*commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(100.0, 1.0)))).into(),
            material: materials.add(ColorMaterial::from(Color::BLACK)),
            transform: Transform::from_xyz(0.0, -5.0, 0.0),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(50.0, 0.5));
    */
    // Add target entity
    commands
        .spawn(TargetBundle::new(Vec3 { x: 4.0, y: 5.0, z: 0.0}, Vec3 { x: 0.5, y: 0.5, z: 0.0}, asset_server));
    
  
}



