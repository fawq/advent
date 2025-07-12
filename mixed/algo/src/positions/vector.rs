use pyo3::prelude::*;

#[pyclass]
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Vector {
    pub add_row: isize,
    pub add_column: isize,
}

#[pymethods]
impl Vector {
    #[new]
    pub fn new(add_row: isize, add_column: isize) -> Self {
        Self {
            add_row,
            add_column,
        }
    }

    pub fn __add__(&self, other: &Self) -> Self {
        Self::new(
            self.add_row + other.add_row,
            self.add_column + other.add_column,
        )
    }

    pub fn __sub__(&self, other: &Self) -> Self {
        Self::new(
            self.add_row - other.add_row,
            self.add_column - other.add_column,
        )
    }

    pub fn scalar_mul(&self, scalar: isize) -> Self {
        Self::new(self.add_row * scalar, self.add_column * scalar)
    }

    pub fn inverse(&self) -> Self {
        Self::new(-self.add_row, -self.add_column)
    }

    pub fn taxicab_norm(&self) -> usize {
        self.add_row.unsigned_abs() + self.add_column.unsigned_abs()
    }

    pub fn __str__(&self) -> String {
        format!("<{}, {}>", self.add_row, self.add_column)
    }

    pub fn __repr__(&self) -> String {
        self.__str__()
    }
}
