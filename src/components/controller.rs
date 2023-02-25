use rlustenv_api::PyDroneDroneController;
use pyo3::prelude::*;
use pyo3::types::PyList;
use std::path::Path;
use std::sync::{Arc, Mutex};

use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct DroneController {
    pub name: Name,
    pub transform: Transform,
    pub is_init: bool,
    pub py_obj: Arc<Mutex<PyDroneDroneController>>,
}


 
impl DroneController {
    pub fn new(name: &str) -> Self {
        let transform = Transform::default();
        Self {
            name: name.into(),
            transform,
            is_init: false,
            py_obj: Arc::new(Mutex::new(PyDroneDroneController::new(transform.translation.into(), String::from(name)))),
        }
    }

    /// Get the thrust of the (left, right) propellers
    pub fn get_thrust(&self) -> (f32, f32) {
        let py_controller = self.py_obj.lock().unwrap();
        (py_controller.thrust_left, py_controller.thrust_right)
    }

    /// Get the angle of the (left, right) propellers
    pub fn get_thrust_angle(&self) -> (f32, f32) {
        let py_controller = self.py_obj.lock().unwrap();
        (py_controller.thrust_left_angle, py_controller.thrust_right_angle)
    }

    /// Update the controller by calling the loop function in the python file or the start function if the controller is not initialized
    pub fn update(&mut self) -> Result<(), PyErr> {
        if !self.is_init {
            info!("Initializing drone controller");
            self.init()?;
            self.is_init = true;
            Ok(())
        } else {
            self.update_()
        }  
    }

    /// Update the properties of the controller
    pub fn update_properties(&mut self, transform: &Transform, velocity: (f32, f32), angular_velocity: f32) {
        self.transform = *transform;
        let mut py_controller = self.py_obj.lock().unwrap();
        py_controller.velocity = velocity;
        py_controller.angular_velocity = angular_velocity;
        py_controller.position = self.transform.translation.into();
    }

    /// Update the controller by calling the loop function in the python file
    fn update_(&mut self) -> Result<(), PyErr> {
        Python::with_gil(|py| {
            let args = (self.py_obj.clone().lock().unwrap().clone(),);
            let controller = py.import("test")?.getattr("loop")?.call1(args)?;
            *self.py_obj.lock().unwrap() = controller.extract()?;
            Ok(())
        })
    }

    /// Initialize the controller by calling the start function in the python file
    fn init(&mut self) -> Result<(), PyErr> {
        //import path for python
        let path = Path::new("./plugins-example/");

        //do useful work
        Python::with_gil(|py| {
            //add the current directory to import path of Python (do not use this in production!)
            let syspath: &PyList = py.import("sys")?.getattr("path")?.extract()?;
            syspath.insert(0, path)?;

            // Now we can load our python_plugin/test.py file.
            // It can in turn import other stuff as it deems appropriate
            let plugin = PyModule::import(py, "test")?;

            let args = (self.py_obj.lock().unwrap().clone(),);
            let controller = plugin.getattr("start")?.call1(args)?;
            *self.py_obj.lock().unwrap() = controller.extract()?;
        
            Ok(())
        })
    }
}