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
        read_lines_to_array2d_f32, read_lines_to_array2d_f64, read_lines_to_array2d_i8,
        read_lines_to_array2d_i16, read_lines_to_array2d_i32, read_lines_to_array2d_i64,
        read_lines_to_array2d_u8, read_lines_to_array2d_u16, read_lines_to_array2d_u32,
        read_lines_to_array2d_u64, read_lines_to_vec, read_lines_to_vec_of_array1d_f32,
        read_lines_to_vec_of_array1d_f64, read_lines_to_vec_of_array1d_i8,
        read_lines_to_vec_of_array1d_i16, read_lines_to_vec_of_array1d_i32,
        read_lines_to_vec_of_array1d_i64, read_lines_to_vec_of_array1d_u8,
        read_lines_to_vec_of_array1d_u16, read_lines_to_vec_of_array1d_u32,
        read_lines_to_vec_of_array1d_u64,
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
fn main_mod(m: &Bound<PyModule>) -> PyResult<()> {
    graphs_mod(m)?;
    matrixes_mod(m)?;
    positions_mod(m)?;
    utils_mod(m)?;

    Ok(())
}

fn graphs_mod(parent: &Bound<PyModule>) -> PyResult<()> {
    let graphs = PyModule::new(parent.py(), "graphs")?;

    graphs.add_class::<Edge>()?;
    graphs.add_class::<GraphType>()?;
    graphs.add_class::<Graph>()?;

    parent.add_submodule(&graphs)?;
    Ok(())
}

fn matrixes_mod(parent: &Bound<PyModule>) -> PyResult<()> {
    let matrixes = PyModule::new(parent.py(), "matrixes")?;

    matrixes.add_class::<BoolMatrix>()?;
    matrixes.add_class::<CharMatrix>()?;
    matrixes.add_class::<I8Matrix>()?;
    matrixes.add_class::<I16Matrix>()?;
    matrixes.add_class::<I32Matrix>()?;
    matrixes.add_class::<I64Matrix>()?;
    matrixes.add_class::<U8Matrix>()?;
    matrixes.add_class::<U16Matrix>()?;
    matrixes.add_class::<U32Matrix>()?;
    matrixes.add_class::<U64Matrix>()?;
    matrixes.add_class::<F32Matrix>()?;
    matrixes.add_class::<F64Matrix>()?;

    parent.add_submodule(&matrixes)?;
    Ok(())
}

fn positions_mod(parent: &Bound<PyModule>) -> PyResult<()> {
    let positions = PyModule::new(parent.py(), "positions")?;

    positions.add_class::<Position>()?;
    positions.add_class::<Direction>()?;
    positions.add_class::<Vector>()?;

    parent.add_submodule(&positions)?;
    Ok(())
}

fn utils_mod(parent: &Bound<PyModule>) -> PyResult<()> {
    let utils = PyModule::new(parent.py(), "utils")?;

    utils.add_function(wrap_pyfunction!(read_lines_to_array2d_i8, &utils)?)?;
    utils.add_function(wrap_pyfunction!(read_lines_to_array2d_i16, &utils)?)?;
    utils.add_function(wrap_pyfunction!(read_lines_to_array2d_i32, &utils)?)?;
    utils.add_function(wrap_pyfunction!(read_lines_to_array2d_i64, &utils)?)?;
    utils.add_function(wrap_pyfunction!(read_lines_to_array2d_u8, &utils)?)?;
    utils.add_function(wrap_pyfunction!(read_lines_to_array2d_u16, &utils)?)?;
    utils.add_function(wrap_pyfunction!(read_lines_to_array2d_u32, &utils)?)?;
    utils.add_function(wrap_pyfunction!(read_lines_to_array2d_u64, &utils)?)?;
    utils.add_function(wrap_pyfunction!(read_lines_to_array2d_f32, &utils)?)?;
    utils.add_function(wrap_pyfunction!(read_lines_to_array2d_f64, &utils)?)?;
    utils.add_function(wrap_pyfunction!(read_lines_to_vec_of_array1d_i8, &utils)?)?;
    utils.add_function(wrap_pyfunction!(read_lines_to_vec_of_array1d_i16, &utils)?)?;
    utils.add_function(wrap_pyfunction!(read_lines_to_vec_of_array1d_i32, &utils)?)?;
    utils.add_function(wrap_pyfunction!(read_lines_to_vec_of_array1d_i64, &utils)?)?;
    utils.add_function(wrap_pyfunction!(read_lines_to_vec_of_array1d_u8, &utils)?)?;
    utils.add_function(wrap_pyfunction!(read_lines_to_vec_of_array1d_u16, &utils)?)?;
    utils.add_function(wrap_pyfunction!(read_lines_to_vec_of_array1d_u32, &utils)?)?;
    utils.add_function(wrap_pyfunction!(read_lines_to_vec_of_array1d_u64, &utils)?)?;
    utils.add_function(wrap_pyfunction!(read_lines_to_vec_of_array1d_f32, &utils)?)?;
    utils.add_function(wrap_pyfunction!(read_lines_to_vec_of_array1d_f64, &utils)?)?;
    utils.add_function(wrap_pyfunction!(read_lines_to_vec, &utils)?)?;

    parent.add_submodule(&utils)?;
    Ok(())
}

define_stub_info_gatherer!(stub_info);
