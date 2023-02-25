use pyo3::prelude::*;


/// A Python class for the object controller
#[derive(Clone)]
#[pyclass(name = "DroneController")]
pub struct PyDroneDroneController {
    #[pyo3(get)]
    pub position: (f32, f32, f32),
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub velocity: (f32, f32),
    #[pyo3(get)]
    pub angular_velocity: f32,
    #[pyo3(get)]
    pub relative_position: (f32, f32),

    #[pyo3(set)]
    pub thrust_left: f32,
    #[pyo3(set)]
    pub thrust_right: f32,
    #[pyo3(set)]
    pub thrust_left_angle: f32,
    #[pyo3(set)]
    pub thrust_right_angle: f32,
}

#[pymethods]
impl PyDroneDroneController {
    #[new]
    pub fn __new__() -> Self {
        Self {
            position: (0.0, 0.0, 0.0),
            name: String::from("unknown"),
            velocity: (0.0, 0.0),
            angular_velocity: 0.0,
            relative_position: (0.0, 0.0),
            thrust_left: 0.0,
            thrust_right: 0.0,
            thrust_left_angle: 0.0,
            thrust_right_angle: 0.0,
        }
    } 
}

impl PyDroneDroneController {
    pub fn new(position: (f32, f32, f32), name: String) -> Self {
        Self {
            position: (position.0, position.1, position.2),
            name,
            velocity: (0.0, 0.0),
            angular_velocity: 0.0,
            relative_position: (0.0, 0.0),
            thrust_left: 0.0,
            thrust_right: 0.0,
            thrust_left_angle: 0.0,
            thrust_right_angle: 0.0,
        }
    }
}


/// A Python module for plugin interface types
#[pymodule]
pub fn rlustenv_api(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyDroneDroneController>()?;
    Ok(())
}