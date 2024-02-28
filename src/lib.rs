pub mod outputs;
pub mod parsers;
use std::{fs, path::Path};

use pyo3::prelude::*;

#[pyfunction]
fn parse(filename: &str, py: Python) -> PyResult<PyObject> {
    let content = fs::read_to_string(Path::new(&filename)).unwrap();
    let states = parsers::parse(content);
    Ok(states.to_object(py))
}

#[pymodule]
fn orca_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    Ok(())
}
