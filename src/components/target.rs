use bevy::prelude::*;

#[derive(Bundle)]
pub struct TargetBundle {
    pub target: Target,
    pub sprite: SpriteBundle,
}

#[derive(Component)]
pub struct Target;

impl TargetBundle {
    /// Create a new target bundle
    pub fn new(position: Vec3, size: Vec3, asset_server: Res<AssetServer>) -> Self {
        Self {
            target: Target,
            sprite: SpriteBundle {
                texture: asset_server.load("images/target.png"),
                transform: Transform::from_xyz(position.x, position.y, position.z),
                sprite: Sprite {
                    custom_size: Some(size.truncate()),
                    ..Default::default()
                },
                ..Default::default()

            }
        }
    }
}
