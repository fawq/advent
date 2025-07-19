use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

macro_rules! create_matrix {
    ($name: ident, $type: ident) => {
        #[gen_stub_pyclass]
        #[pyclass]
        pub struct $name {
            matrix: Vec<Vec<$type>>,
            bound_height: usize,
            bound_width: usize,
        }

        #[gen_stub_pymethods]
        #[pymethods]
        impl $name {
            #[new]
            pub fn new(bound_height: usize, bound_width: usize) -> Self {
                Self {
                    matrix: Vec::new(),
                    bound_height,
                    bound_width,
                }
            }
        }
    };
}

create_matrix!(IntMatrix, i32);
create_matrix!(BoolMatrix, bool);
create_matrix!(StringMatrix, String);
create_matrix!(FloatMatrix, f64);
