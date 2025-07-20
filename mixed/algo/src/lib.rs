mod graphs;
mod matrixes;
mod positions;
use crate::graphs::{
    edge::Edge,
    graph::{Graph, GraphType},
};
use crate::{
    matrixes::generic_matrix::{BoolMatrix, CharMatrix, FloatMatrix, IntMatrix},
    positions::{
        position::{Direction, Position},
        vector::Vector,
    },
};
use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;

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
    m.add_class::<BoolMatrix>()?;
    m.add_class::<IntMatrix>()?;
    m.add_class::<FloatMatrix>()?;
    m.add_class::<CharMatrix>()?;
    Ok(())
}

define_stub_info_gatherer!(stub_info);
