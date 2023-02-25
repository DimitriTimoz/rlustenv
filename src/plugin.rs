use crate::{prelude::*, ui::pages::*};
use bevy::{prelude::*, winit::WinitSettings, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_rapier2d::prelude::*;

use crate::components::controller::DroneController;

pub struct RlustenvPlugin;

impl Plugin for RlustenvPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(FrameTimeDiagnosticsPlugin)
            .add_plugin(EguiPlugin)
            .add_plugin(DebugLinesPlugin::default())
            .add_plugin(DefaultInspectorConfigPlugin)
            .insert_resource(Zoom::default())    
            .insert_resource(WinitSettings::desktop_app())
            .add_event::<CursorMoved>()
            .add_startup_system(Self::setup_camera)
            .add_startup_system(setup_main_ui)
            .add_system(DroneBundle::update_drone)
            .add_system(change_fps_system)
            .add_system(zoom_system)
            .add_system(update_hierachy)
            .add_system(Self::update_controllers);
        pyo3::append_to_inittab!(pylib_module);
    }    
}


impl RlustenvPlugin {

    fn setup_camera(mut commands: Commands) {
        // Add a camera so we can see the debug-render.
        #[cfg(not(feature = "3d"))]
        commands.spawn(Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            projection: OrthographicProjection {
                scale: 1./50.,

                ..Default::default()
            },
            ..Default::default()
        });
        #[cfg(feature = "3d")]
        commands.spawn(Camera3dBundle::default());
    }

    fn update_controllers(mut controllers: Query<(&mut DroneController, &Transform)>) {
        for (mut controller, transform) in controllers.iter_mut() {
            controller.update_position(transform);
            match controller.update() {
                Ok(_) => {}
                Err(e) => {
                    error!("Error: {e:?}");
                }
            }
        }
    }

}
