use bevy::{input::mouse::MouseWheel, prelude::*};

const CAMERA_SPEED_PER_SEC: f32 = 1.0;

pub fn zoom_system(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    windows: Res<Windows>,
    mut zoom: ResMut<Zoom>,
    mouse_position: Res<Events<CursorMoved>>,
    mut query: Query<(&mut Transform, &mut Camera)>,
) {


    for event in mouse_wheel_events.iter() {
        for (mut transform, mut camera) in query.iter_mut() {
            let window = windows.get_primary().expect("no primary window");
            let aspect_ratio = window.width() / window.height();
            let zoom_delta = event.y * zoom.speed;
            let new_zoom = (transform.scale.x - zoom_delta).clamp(zoom.min, zoom.max);
            transform.scale = Vec3::new(new_zoom, new_zoom / aspect_ratio, 1.0);
        }
    }
}

#[derive(Resource)]
pub struct Zoom {
    pub speed: f32,
    pub min: f32,
    pub max: f32,
}

impl Default for Zoom {
    fn default() -> Self {
        Zoom {
            speed: 0.0005,
            min: 0.1,
            max: 10.0,
        }
    }
}

