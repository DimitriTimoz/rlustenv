use bevy::prelude::*;

#[derive(Component)]
pub struct FpsText;


pub fn setup_fps_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Root node
    commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(0.0),
                bottom: Val::Px(0.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }).with_children(|parent| {
            parent.spawn((TextBundle {
                text: Text::from_sections(
                    vec![
                        TextSection {
                            value: "0.0".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 14.0,
                                color: Color::BLACK,
                            },
                        },
                        TextSection {
                            value: " fps".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 14.0,
                                color: Color::BLACK,
                            },
                        },
                    ],
                ),
                ..Default::default()
            },FpsText));
        });
    });
}