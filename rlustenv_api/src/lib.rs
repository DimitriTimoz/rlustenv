use pyo3::prelude::*;


/// A Python class for the object controller
#[derive(Clone)]
#[pyclass(name = "Controller")]
pub struct PyController {
    #[pyo3(get)]
    pub position: (f32, f32, f32),
}

#[pymethods]
impl PyController {
    #[new]
    pub fn __new__() -> Self {
        Self {
            position: (0.0, 0.0, 0.0),
        }
    } 
}

impl PyController {
    pub fn new(position: (f32, f32, f32),) -> Self {
        Self {
            position: (position.0, position.1, position.2),
        }
    }
}


/// A Python module for plugin interface types
#[pymodule]
pub fn rlustenv_api(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyController>()?;
    Ok(())
}