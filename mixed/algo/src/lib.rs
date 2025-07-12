mod graphs;
mod positions;
use crate::graphs::{
    edge::Edge,
    graph::{Graph, GraphType},
};
use crate::positions::{
    position::{Direction, Position},
    vector::Vector,
};
use pyo3::prelude::*;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Edge>()?;
    m.add_class::<GraphType>()?;
    m.add_class::<Graph>()?;
    m.add_class::<Vector>()?;
    m.add_class::<Position>()?;
    m.add_class::<Direction>()?;
    Ok(())
}
