mod graphs;
mod matrixes;
mod positions;
mod utils;
use crate::graphs::{
    edge::Edge,
    graph::{Graph, GraphType},
};
use crate::{
    matrixes::matrix::{
        BoolMatrix, CharMatrix, F32Matrix, F64Matrix, I8Matrix, I16Matrix, I32Matrix, I64Matrix,
        U8Matrix, U16Matrix, U32Matrix, U64Matrix,
    },
    positions::{
        position::{Direction, Position},
        vector::Vector,
    },
    utils::file::{
        read_lines_to_vec_f32, read_lines_to_vec_f64, read_lines_to_vec_i8, read_lines_to_vec_i16,
        read_lines_to_vec_i32, read_lines_to_vec_i64, read_lines_to_vec_u8, read_lines_to_vec_u16,
        read_lines_to_vec_u32, read_lines_to_vec_u64,
    },
};
use pyo3::{
    Bound, PyResult, pymodule,
    types::{PyModule, PyModuleMethods},
    wrap_pyfunction,
};
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
    m.add_class::<CharMatrix>()?;
    m.add_class::<I8Matrix>()?;
    m.add_class::<I16Matrix>()?;
    m.add_class::<I32Matrix>()?;
    m.add_class::<I64Matrix>()?;
    m.add_class::<U8Matrix>()?;
    m.add_class::<U16Matrix>()?;
    m.add_class::<U32Matrix>()?;
    m.add_class::<U64Matrix>()?;
    m.add_class::<F32Matrix>()?;
    m.add_class::<F64Matrix>()?;

    m.add_function(wrap_pyfunction!(read_lines_to_vec_i8, m)?)?;
    m.add_function(wrap_pyfunction!(read_lines_to_vec_i16, m)?)?;
    m.add_function(wrap_pyfunction!(read_lines_to_vec_i32, m)?)?;
    m.add_function(wrap_pyfunction!(read_lines_to_vec_i64, m)?)?;
    m.add_function(wrap_pyfunction!(read_lines_to_vec_u8, m)?)?;
    m.add_function(wrap_pyfunction!(read_lines_to_vec_u16, m)?)?;
    m.add_function(wrap_pyfunction!(read_lines_to_vec_u32, m)?)?;
    m.add_function(wrap_pyfunction!(read_lines_to_vec_u64, m)?)?;
    m.add_function(wrap_pyfunction!(read_lines_to_vec_f32, m)?)?;
    m.add_function(wrap_pyfunction!(read_lines_to_vec_f64, m)?)?;

    Ok(())
}

define_stub_info_gatherer!(stub_info);
