use pyo3::{pyclass, pymethods};
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

#[gen_stub_pyclass]
#[pyclass(module = "algo._core.graphs")]
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Edge {
    pub from_node: usize,
    pub to_node: usize,
}

#[gen_stub_pymethods]
#[pymethods]
impl Edge {
    #[new]
    pub fn new(from_node: usize, to_node: usize) -> Self {
        Self { from_node, to_node }
    }

    pub fn __str__(&self) -> String {
        format!("({} -> {})", self.from_node, self.to_node)
    }

    pub fn __repr__(&self) -> String {
        self.__str__()
    }

    pub fn reverse(&self) -> Self {
        Self {
            from_node: self.to_node,
            to_node: self.from_node,
        }
    }
}
