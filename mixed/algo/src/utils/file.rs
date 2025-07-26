use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

#[gen_stub_pyfunction]
#[pyfunction]
pub fn read_lines(file_path: &str) -> Vec<String> {
    match std::fs::read_to_string(file_path) {
        Ok(content) => content.lines().map(String::from).collect(),
        Err(e) => panic!("Error reading file {file_path}: {e}"),
    }
}

fn read_vec<T: std::str::FromStr>(line: &str) -> Vec<T> {
    line.split_ascii_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn read_double<T: std::str::FromStr>(line: &str) -> (T, T) {
    let mut iter = line.split_ascii_whitespace();
    let t = iter.next().unwrap().parse().ok().unwrap();
    let s = iter.next().unwrap().parse().ok().unwrap();
    (t, s)
}

fn read_triple<T: std::str::FromStr>(line: &str) -> (T, T, T) {
    let mut iter = line.split_ascii_whitespace();
    let t = iter.next().unwrap().parse().ok().unwrap();
    let s = iter.next().unwrap().parse().ok().unwrap();
    let r = iter.next().unwrap().parse().ok().unwrap();
    (t, s, r)
}

fn read_lines_to_vec<T: std::str::FromStr>(file_path: &str) -> Vec<Vec<T>> {
    read_lines(file_path)
        .iter()
        .map(|line| read_vec(line))
        .collect()
}

fn read_lines_to_double<T: std::str::FromStr>(file_path: &str) -> Vec<(T, T)> {
    read_lines(file_path)
        .iter()
        .map(|line| read_double(line))
        .collect()
}
