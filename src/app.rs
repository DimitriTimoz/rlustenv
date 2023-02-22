use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct App {
    app: bevy::app::App,
}

impl App {
    pub fn new() -> Self {
        let mut app = bevy::app::App::new();
        app.add_plugins(DefaultPlugins)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(RapierDebugRenderPlugin::default());

        Self { app }
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

    pub fn run(&mut self) {
        self.app.run();
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
