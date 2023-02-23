use crate::{prelude::*, ui::pages::*};
use bevy::{prelude::*, winit::WinitSettings, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_rapier2d::prelude::*;

use crate::components::controller::Controller;

pub struct App {
    app: bevy::app::App,
}

impl App {
    pub fn new() -> Self {
        let mut app = bevy::app::App::new();
        app.add_plugins(DefaultPlugins)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(FrameTimeDiagnosticsPlugin)
            .insert_resource(WinitSettings::desktop_app())
            .add_startup_system(Self::setup_camera);
        pyo3::append_to_inittab!(pylib_module);

        Self { app }
    }

    pub fn run(&mut self) {
        self.add_ui_startup_system();
        self.add_ui_system();
        self.app.add_system(Self::update_controllers).run();
    }

    pub fn add_startup_system<Params>(
        &mut self,
        system: impl IntoSystemDescriptor<Params>,
    ) -> &mut Self {
        self.app.add_startup_system(system);
        self
    }

    pub fn add_system<Params>(&mut self, system: impl IntoSystemDescriptor<Params>) -> &mut Self {
        self.app.add_system_to_stage(CoreStage::Update, system);
        self
    }


    pub fn add_plugin(&mut self, plugin: impl Plugin) -> &mut Self {
        self.app.add_plugin(plugin);
        self
    }

    pub fn add_plugins(&mut self, plugins: impl PluginGroup) -> &mut Self {
        self.app.add_plugins(plugins);
        self
    }

    fn add_ui_startup_system(&mut self) {
        self.app.add_startup_system(setup_main_ui);
    }

    fn add_ui_system(&mut self) {
        self.app.add_system_to_stage(CoreStage::Update, mouse_scroll);
        self.app.add_system_to_stage(CoreStage::Update, change_fps_system);
    }
    

    fn setup_camera(mut commands: Commands) {
        // Add a camera so we can see the debug-render.
        #[cfg(not(feature = "3d"))]
        commands.spawn(Camera2dBundle::default());
        #[cfg(feature = "3d")]
        commands.spawn(Camera3dBundle::default());
    }

    fn update_controllers(mut controllers: Query<(&mut Controller, &Transform)>) {
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

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
