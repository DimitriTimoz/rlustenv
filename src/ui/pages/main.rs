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
            // right vertical fill
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        size: Size::new(Val::Px(200.0), Val::Auto),
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn(
                        TextBundle::from_section(
                            "Scrolling list",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 25.,
                                color: Color::WHITE,
                            },
                        )
                        .with_style(Style {
                            size: Size::new(Val::Auto, Val::Px(25.)),
                            ..default()
                        }),
                    );
                    // List with hidden overflow
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_self: AlignSelf::Stretch,
                                size: Size::new(Val::Auto, Val::Percent(50.0)),
                                overflow: Overflow::Hidden,
                                ..default()
                            },
                            background_color: Color::rgb(0.10, 0.10, 0.10).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // Moving panel
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            flex_direction: FlexDirection::Column,
                                            flex_grow: 1.0,
                                            max_size: Size::UNDEFINED,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    ScrollingList::default(),
                                ))
                                .with_children(|parent| {
                                    // List items
                                    for i in 0..30 {
                                        parent.spawn(
                                            TextBundle::from_section(
                                                format!("Item {i}"),
                                                TextStyle {
                                                    font: asset_server
                                                        .load("fonts/FiraSans-Bold.ttf"),
                                                    font_size: 20.,
                                                    color: Color::WHITE,
                                                },
                                            )
                                            .with_style(Style {
                                                flex_shrink: 0.,
                                                size: Size::new(Val::Undefined, Val::Px(20.)),
                                                ..default()
                                            }),
                                        );
                                    }
                                });
                        });
                });
            parent.spawn(setup_fps_ui(asset_server));
        });
}
