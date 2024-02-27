pub mod outputs;
pub mod parser;
use std::{collections::HashMap, fs, path::Path};

use crate::parser::parse;

use pyo3::prelude::*;

#[pyfunction]
fn pyparse(filename: &str, py: Python) -> PyResult<Vec<PyObject>> {
    let content = fs::read_to_string(Path::new(&filename)).unwrap();
    let mut iterator = content.lines();
    while !iterator.next().unwrap().starts_with("Eigenvectors") {}
    iterator.next();
    let states = parse(&iterator.collect());
    Ok(states.iter().map(|x| x.to_object(py)).collect())
}
/// A Python module implemented in Rust.
#[pymodule]
fn orca_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pyparse, m)?)?;
    Ok(())
}
