use std::collections::HashMap;

use pyo3::prelude::*;
use pyo3::ToPyObject;

#[derive(Debug)]
pub struct Contribution {
    pub atom_nr: u32,
    pub element: String,
    pub orbital_type: String,
    pub contribution: f32,
}

#[derive(Debug)]
pub struct Orbital {
    pub orbital_nr: u32,
    pub energy: f32,
    pub occupation: f32,
    pub contributions: Vec<Contribution>,
}

impl ToPyObject for Contribution {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        let mut obj = HashMap::<String, PyObject>::new();
        obj.insert("atom_nr".to_string(), self.atom_nr.to_object(py));
        obj.insert("element".to_string(), self.element.to_object(py));
        obj.insert("orbital_type".to_string(), self.orbital_type.to_object(py));
        obj.insert("contribution".to_string(), self.contribution.to_object(py));
        obj.to_object(py)
    }
}

impl ToPyObject for Orbital {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        let mut obj = HashMap::<String, PyObject>::new();
        obj.insert("orbital_nr".to_string(), self.orbital_nr.to_object(py));
        obj.insert("energy".to_string(), self.energy.to_object(py));
        obj.insert("occupation".to_string(), self.occupation.to_object(py));
        obj.insert(
            "contributions".to_string(),
            self.contributions.to_object(py),
        );
        obj.to_object(py)
    }
}

#[derive(Debug)]
pub struct CASSCFOutput {
    pub orbital_contributions: Vec<Orbital>,
}
impl ToPyObject for CASSCFOutput {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        let mut obj = HashMap::<String, PyObject>::new();
        obj.insert(
            "orbital_contributions".to_string(),
            self.orbital_contributions.to_object(py),
        );
        obj.to_object(py)
    }
}
