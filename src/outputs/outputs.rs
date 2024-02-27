use std::collections::HashMap;

use pyo3::ToPyObject;
use pyo3::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct SOCEigenvector {
    pub weight: f32,
    pub real: f32,
    pub imag: f32,
    pub root: u8,
    pub spin: u8,
    pub ms: i8,
}


impl ToPyObject for SOCEigenvector {
    fn to_object(&self, py: pyo3::prelude::Python<'_>) -> pyo3::prelude::PyObject {
        let mut obj = HashMap::<String, PyObject>::new();
        obj.insert("weight".to_string(), self.weight.to_object(py));
        obj.insert("real".to_string(), self.real.to_object(py));
        obj.insert("imag".to_string(), self.imag.to_object(py));
        obj.insert("root".to_string(), self.root.to_object(py));
        obj.insert("spin".to_string(), self.spin.to_object(py));
        obj.insert("ms".to_string(), self.ms.to_object(py));
        obj.to_object(py)
    }
}


#[derive(Clone, Debug)]
pub struct SOCState {
    pub n: u32,
    pub energy: f32,
    pub eigenvectors: Vec<SOCEigenvector>,
}

impl ToPyObject for SOCState {
    fn to_object(&self, py: pyo3::prelude::Python<'_>) -> pyo3::prelude::PyObject {
        let mut obj = HashMap::<String, PyObject>::new();
        obj.insert("n".to_string(), self.n.to_object(py));
        obj.insert("energy".to_string(), self.energy.to_object(py));
        obj.insert("eigenvectors".to_string(), self.eigenvectors.to_object(py));
        obj.to_object(py)
    }
}
