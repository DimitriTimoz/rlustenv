use rlustenv_api::PyController;
use pyo3::prelude::*;
use pyo3::types::PyList;
use std::path::Path;
use std::sync::{Arc, Mutex};

use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Controller {
    pub name: Name,
    pub transform: TransformBundle,
    pub is_init: bool,
    pub py_obj: Arc<Mutex<PyController>>,
}


impl Default for Controller {
    fn default() -> Self {
        let transform = TransformBundle::default();
        Self {
            name: Name::new("Controller"),
            transform,
            is_init: false,
            py_obj: Arc::new(Mutex::new(PyController::new(transform.local.translation.into()))),
        }
    }
}

impl Controller {
    pub fn update(&mut self) -> Result<(), PyErr> {
        if !self.is_init {
            info!("Initializing controller");
            self.init()?;
            self.is_init = true;
            Ok(())
        } else {
            self.update_()
        }  
    }
    pub fn update_data(&mut self, transform: &Transform) {

        *self.py_obj.lock().unwrap() = PyController::new(self.transform.local.translation.into());   
    
    }
    fn update_(&mut self) -> Result<(), PyErr> {
        Python::with_gil(|py| {
            let args = (self.py_obj.clone().lock().unwrap().clone(),);
            let controller = py.import("test")?.getattr("loop")?.call1(args)?;
            println!("{:?}", controller.getattr("position"));
            *self.py_obj.lock().unwrap() = controller.extract()?;
            Ok(())
        })
    }

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
        
            // any modifications we make to rust object are reflected on Python object as well
            let res: (f32, f32, f32) = controller.getattr("position")?.extract()?;
            println!("{:?}", res);
            Ok(())
        })
    }
}