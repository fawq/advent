use crate::positions::vector::Vector;
use pyo3::{pyclass, pymethods};
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pyclass_enum, gen_stub_pymethods};

#[gen_stub_pyclass_enum]
#[pyclass(eq, eq_int, module = "algo.main_mod.positions")]
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for Vector {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => Vector::new(-1, 0),
            Direction::Down => Vector::new(1, 0),
            Direction::Left => Vector::new(0, -1),
            Direction::Right => Vector::new(0, 1),
        }
    }
}

#[gen_stub_pyclass]
#[pyclass(module = "algo.main_mod.positions")]
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

#[gen_stub_pymethods]
#[pymethods]
impl Position {
    #[new]
    pub fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    pub fn __str__(&self) -> String {
        format!("[{}, {}]", self.row, self.column)
    }

    pub fn __repr__(&self) -> String {
        self.__str__()
    }

    pub fn get_new_position_with_vector(&self, vector: Vector) -> Position {
        Position::new(
            self.row.saturating_add_signed(vector.add_row),
            self.column.saturating_add_signed(vector.add_column),
        )
    }

    pub fn get_new_position_with_direction(&self, direction: Direction) -> Position {
        self.get_new_position_with_vector(direction.into())
    }

    pub fn set_new_position_with_vector(&mut self, vector: Vector) {
        self.row = self.row.saturating_add_signed(vector.add_row);
        self.column = self.column.saturating_add_signed(vector.add_column);
    }

    pub fn set_new_position_with_direction(&mut self, direction: Direction) {
        self.set_new_position_with_vector(direction.into())
    }

    pub fn vector_to(&self, other: &Position) -> Vector {
        Vector::new(
            other.row as isize - self.row as isize,
            other.column as isize - self.column as isize,
        )
    }
}
