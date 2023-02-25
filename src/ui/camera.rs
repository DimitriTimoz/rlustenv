use bevy::{input::mouse::{MouseWheel, MouseMotion}, prelude::*};

const CAMERA_SPEED_PER_SEC: f32 = 1.0;

fn zoom_system(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    windows: Res<Windows>,
    mouse_position: Res<Events<CursorMoved>>,
    mut query: Query<(&mut Transform, &mut Camera, &mut Camera2D)>,
) {
    for event in mouse_wheel_events.iter() {
        for (mut transform, mut camera, cam2d) in query.iter_mut() {
            let window = windows.get_primary().expect("no primary window");
            let aspect_ratio = window.width() / window.height();
            let zoom_delta = event.y * cam2d.zoom_speed;
            let new_zoom = (transform.scale.x - zoom_delta).clamp(cam2d.min, cam2d.max);
            transform.scale = Vec3::new(new_zoom, new_zoom / aspect_ratio, 1.0);
        }
    }
}

fn move_system(
    time: Res<Time>,
    mouse_input: Res<Input<MouseButton>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &Camera2D)>
) {
    for (mut transform, mut camera) in query.iter_mut() {
        let mut translation = transform.translation;

        if mouse_input.pressed(MouseButton::Left) {
            if let Some(motion_evr) = motion_evr.iter().last() {
                let delta = time.delta_seconds() * camera.speed;
                let mouse_delta = Vec2::new(
                    -delta * motion_evr.delta.x,
                    delta * motion_evr.delta.y,
                );
                translation += mouse_delta.extend(0.0);
    
            }
        }
        transform.translation = translation;
    }
}

pub fn camera_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(zoom_system)
        .with_system(move_system)
}

#[derive(Component)]
pub struct Camera2D {
    pub zoom_speed: f32,
    pub min: f32,
    pub max: f32,
    pub speed: f32,
}

impl Default for Camera2D {
    fn default() -> Self {
        Camera2D {
            zoom_speed: 0.0005,
            min: 0.1,
            max: 10.0,
            speed: 1.0,
        }
    }
}

