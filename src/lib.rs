pub mod outputs;
pub mod parsers;
use std::{fs, path::Path};

use pyo3::prelude::*;

#[pyfunction]
fn parse_tddft(filename: &str, py: Python) -> PyResult<PyObject> {
    let content = fs::read_to_string(Path::new(&filename)).unwrap();
    let states = parsers::parse_tddft(content);
    Ok(states.to_object(py))
}


#[pyfunction]
fn parse_casscf(filename: &str, py: Python) -> PyResult<PyObject> {
    let content = fs::read_to_string(Path::new(&filename)).unwrap();
    let states = parsers::parse_casscf(content);
    Ok(states.to_object(py))
}

#[pymodule]
fn orca_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_tddft, m)?)?;
    m.add_function(wrap_pyfunction!(parse_casscf, m)?)?;
    Ok(())
}
