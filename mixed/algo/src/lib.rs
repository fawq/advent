mod graphs;
use crate::graphs::{edge::Edge, graph::Graph};
use pyo3::prelude::*;

#[pyfunction]
fn hello_from_bin() -> String {
    "Hello from algo method in rust!".to_string()
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello_from_bin, m)?)?;
    m.add_class::<Edge>()?;
    m.add_class::<Graph>()?;
    Ok(())
}
