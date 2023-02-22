use rlustenv_api::PyController;
use pyo3::prelude::*;
use pyo3::types::PyList;
use std::path::Path;

use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Controller {
    obj: PyController,
    is_init: bool,
}


impl Default for Controller {
    fn default() -> Self {
        Self {
            obj: PyController::__new__(),
            is_init: false,
        }
    }
}

impl Controller {
    
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
    fn update_(&mut self) -> Result<(), PyErr> {
        Python::with_gil(|py| {
            let args = (self.obj.clone(),);
            let controller = py.import("test")?.getattr("loop")?.call1(args)?;
            println!("{:?}", controller.getattr("id"));
            self.obj = controller.extract()?;
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

            let args = (self.obj.clone(),);
            let controller = plugin.getattr("start")?.call1(args)?;
            self.obj = controller.extract()?;
        
            // any modifications we make to rust object are reflected on Python object as well
            let res: usize = controller.getattr("id")?.extract()?;
            println!("{res}");
            Ok(())
        })
    }
}