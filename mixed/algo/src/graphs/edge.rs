use pyo3::prelude::*;

#[pyclass]
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
}

#[pymethods]
impl Edge {
    #[new]
    pub fn new(from: usize, to: usize) -> Self {
        Self { from, to }
    }

    pub fn reverse(&self) -> Self {
        Self {
            from: self.to,
            to: self.from,
        }
    }

    pub fn __str__(&self) -> String {
        format!("({} -> {})", self.from, self.to)
    }

    pub fn __repr__(&self) -> String {
        self.__str__()
    }
}
