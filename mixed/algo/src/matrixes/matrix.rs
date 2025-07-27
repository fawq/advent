use pyo3::{pyclass, pymethods};
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

use crate::positions::position::Position;
use numpy::ndarray::Array2;

macro_rules! create_matrix {
    ($name: ident, $type: ident) => {
        #[gen_stub_pyclass]
        #[pyclass]
        pub struct $name {
            matrix: Array2<$type>,
            height: usize,
            width: usize,
        }

        #[gen_stub_pymethods]
        #[pymethods]
        impl $name {
            #[new]
            pub fn new(height: usize, width: usize) -> Self {
                Self {
                    matrix: Array2::default((height, width)),
                    height,
                    width,
                }
            }

            pub fn __str__(&self) -> String {
                let mut result = String::new();
                for row in self.matrix.rows() {
                    for element in row {
                        result.push_str(&element.to_string());
                    }
                    result.push('\n');
                }

                result
            }

            pub fn __repr__(&self) -> String {
                self.__str__()
            }

            pub fn get_height(&self) -> usize {
                self.height
            }

            pub fn get_width(&self) -> usize {
                self.width
            }

            pub fn set_element(&mut self, row: usize, column: usize, value: $type) {
                if !self.is_in_bounds(row, column) {
                    panic!("Out of bounds");
                }

                match self.matrix.get_mut((row, column)) {
                    Some(element) => *element = value,
                    None => panic!("Out of bounds after indexing"),
                }
            }

            pub fn set_position_value(&mut self, position: Position, value: $type) {
                self.set_element(position.row, position.column, value);
            }

            pub fn get_element(&self, row: usize, column: usize) -> $type {
                if !self.is_in_bounds(row, column) {
                    panic!("Out of bounds");
                }

                match self.matrix.get((row, column)) {
                    Some(element) => *element,
                    None => panic!("Out of bounds after indexing"),
                }
            }

            pub fn get_position_value(&self, position: Position) -> $type {
                self.get_element(position.row, position.column)
            }

            pub fn is_in_bounds(&self, row: usize, column: usize) -> bool {
                row < self.height && column < self.width
            }
        }
    };
}

create_matrix!(I8Matrix, i8);
create_matrix!(I16Matrix, i16);
create_matrix!(I32Matrix, i32);
create_matrix!(I64Matrix, i64);
create_matrix!(U8Matrix, u8);
create_matrix!(U16Matrix, u16);
create_matrix!(U32Matrix, u32);
create_matrix!(U64Matrix, u64);
create_matrix!(F32Matrix, f32);
create_matrix!(F64Matrix, f64);
create_matrix!(BoolMatrix, bool);
create_matrix!(CharMatrix, char);
