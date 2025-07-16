use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

const MAX_HEIGHT: usize = 10;
const MAX_WIDTH: usize = 10;

#[gen_stub_pyclass]
#[pyclass]
#[derive(Hash, Eq, PartialEq, Debug)]
pub struct StaticMatrix {
    matrix: [[i64; MAX_WIDTH]; MAX_HEIGHT],
    bound_height: usize,
    bound_width: usize,
}

#[gen_stub_pymethods]
#[pymethods]
impl StaticMatrix {
    #[new]
    pub fn new(default_value: i64, bound_height: usize, bound_width: usize) -> Self {
        Self {
            matrix: [[default_value; MAX_WIDTH]; MAX_HEIGHT],
            bound_height,
            bound_width,
        }
    }
}
