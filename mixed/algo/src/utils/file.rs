use numpy::IntoPyArray;
use numpy::array::PyArray1;
use numpy::array::PyArray2;
use numpy::ndarray::{Array1, Array2, Axis, stack};
use pyo3::{Bound, Python, pyfunction};
use pyo3_stub_gen::derive::gen_stub_pyfunction;

fn read_lines(file_path: &str) -> Array1<String> {
    let all_lines = match std::fs::read_to_string(file_path) {
        Ok(content) => content.lines().map(|line| line.to_string()).collect(),
        Err(e) => panic!("Error reading file {file_path}: {e}"),
    };

    Array1::from_vec(all_lines)
}

fn read_array<T: std::str::FromStr + Clone>(line: &str) -> Array1<T> {
    line.split_ascii_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn read_lines_to_array2d<T: std::str::FromStr + Clone>(file_path: &str) -> Array2<T> {
    let lines = read_lines(file_path);
    let arrays: Vec<Array1<T>> = lines.iter().map(|line| read_array(line)).collect();
    stack(
        Axis(0),
        &arrays.iter().map(|a| a.view()).collect::<Vec<_>>(),
    )
    .unwrap()
}

#[gen_stub_pyfunction(module = "algo._core.utils")]
#[pyfunction]
pub fn read_lines_to_vec(file_path: &str) -> Vec<String> {
    read_lines(file_path).to_vec()
}

macro_rules! create_read_lines_to_array2d {
    ($name: ident, $type: ident) => {
        #[gen_stub_pyfunction(module = "algo._core.utils")]
        #[pyfunction]
        pub fn $name<'py>(py: Python<'py>, file_path: &str) -> Bound<'py, PyArray2<$type>> {
            read_lines_to_array2d::<$type>(file_path).into_pyarray(py)
        }
    };
}

macro_rules! create_read_lines_to_vec_of_array1d {
    ($name: ident, $type: ident) => {
        #[gen_stub_pyfunction(module = "algo._core.utils")]
        #[pyfunction]
        pub fn $name<'py>(py: Python<'py>, file_path: &str) -> Vec<Bound<'py, PyArray1<$type>>> {
            let lines = read_lines(file_path);
            let arrays = lines
                .iter()
                .map(|line| read_array(line).into_pyarray(py))
                .collect();
            arrays
        }
    };
}

create_read_lines_to_array2d!(read_lines_to_array2d_i8, i8);
create_read_lines_to_array2d!(read_lines_to_array2d_i16, i16);
create_read_lines_to_array2d!(read_lines_to_array2d_i32, i32);
create_read_lines_to_array2d!(read_lines_to_array2d_i64, i64);
create_read_lines_to_array2d!(read_lines_to_array2d_u8, u8);
create_read_lines_to_array2d!(read_lines_to_array2d_u16, u16);
create_read_lines_to_array2d!(read_lines_to_array2d_u32, u32);
create_read_lines_to_array2d!(read_lines_to_array2d_u64, u64);
create_read_lines_to_array2d!(read_lines_to_array2d_f32, f32);
create_read_lines_to_array2d!(read_lines_to_array2d_f64, f64);

create_read_lines_to_vec_of_array1d!(read_lines_to_vec_of_array1d_i8, i8);
create_read_lines_to_vec_of_array1d!(read_lines_to_vec_of_array1d_i16, i16);
create_read_lines_to_vec_of_array1d!(read_lines_to_vec_of_array1d_i32, i32);
create_read_lines_to_vec_of_array1d!(read_lines_to_vec_of_array1d_i64, i64);
create_read_lines_to_vec_of_array1d!(read_lines_to_vec_of_array1d_u8, u8);
create_read_lines_to_vec_of_array1d!(read_lines_to_vec_of_array1d_u16, u16);
create_read_lines_to_vec_of_array1d!(read_lines_to_vec_of_array1d_u32, u32);
create_read_lines_to_vec_of_array1d!(read_lines_to_vec_of_array1d_u64, u64);
create_read_lines_to_vec_of_array1d!(read_lines_to_vec_of_array1d_f32, f32);
create_read_lines_to_vec_of_array1d!(read_lines_to_vec_of_array1d_f64, f64);
