use pyo3::prelude::*;


#[pyfunction] pub(crate) 
fn is_open() -> bool {
    return true
}