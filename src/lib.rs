pub mod outputs;
pub mod parsers;
use std::{fs, path::Path};

use pyo3::prelude::*;

#[pyfunction]
fn parse(filename: &str, py: Python) -> PyResult<Vec<PyObject>> {
    let content = fs::read_to_string(Path::new(&filename)).unwrap();

    // let mut iterator = content.lines();
    // while !iterator.next().unwrap().starts_with("Eigenvectors") {}
    // iterator.next();
    let states = parsers::parse(content).unwrap();
    Ok(states.iter().map(|x| x.to_object(py)).collect())
}

#[pymodule]
fn orca_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    Ok(())
}
