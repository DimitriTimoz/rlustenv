use bevy::prelude::*;

use crate::prelude::*;

use super::setup_fps_ui;

pub fn setup_main_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Root node
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Auto),
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(setup_fps_ui(asset_server));
        });
}
