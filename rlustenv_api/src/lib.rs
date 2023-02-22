use pyo3::prelude::*;

/// A Python class for the object controller
#[derive(Clone)]
#[pyclass(name = "Controller")]
pub struct PyController {
    #[pyo3(get, set)]
    pub id: u64,
    //this field will only be accessible to rust code
    pub rustonly: Vec<usize>,
}

#[pymethods]
impl PyController {
    #[new]
    pub fn __new__() -> Self {
        Self {
            id: 0,
            rustonly: Vec::new(),
        }
    } 
}



/// A Python module for plugin interface types
#[pymodule]
pub fn rlustenv_api(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyController>()?;
    Ok(())
}