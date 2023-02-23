use bevy::{prelude::*, diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin}};
use crate::prelude::*;

pub fn setup_fps_ui(asset_server: Res<AssetServer>) -> (TextBundle, FpsText) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    (TextBundle::from_sections([
        TextSection::from_style(TextStyle {
            font: font.clone(),
            font_size: 14.0,
            color: Color::BLACK,
        }),
        TextSection::new(
            " fps, ",
            TextStyle {
                font: font.clone(),
                font_size: 14.0,
                color: Color::BLACK,
            },
        ),
        TextSection::from_style(TextStyle {
            font: font.clone(),
            font_size: 14.0,
            color: Color::BLACK,
        }),
        TextSection::new(
            " ms/frame",
            TextStyle {
                font,
                font_size: 14.0,
                color: Color::BLACK,
            },
        ),
    ]),
    FpsText)
}


pub fn change_fps_system(
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        let mut fps = 0.0;
        if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_smoothed) = fps_diagnostic.smoothed() {
                fps = fps_smoothed;
            }
        }

        let mut frame_time = time.delta_seconds_f64();
        if let Some(frame_time_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
        {
            if let Some(frame_time_smoothed) = frame_time_diagnostic.smoothed() {
                frame_time = frame_time_smoothed;
            }
        }

 
        text.sections[0].value = format!("{fps:.1}");

        text.sections[2].value = format!("{frame_time:.3}");
    }
}
