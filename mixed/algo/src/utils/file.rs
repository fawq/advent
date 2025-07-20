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

fn read<T>(line: &String) -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    match line.trim().parse() {
        Ok(t) => t,
        Err(e) => panic!("Error parsing line {line}: {e}"),
    }
}

fn read_vec<T>(line: &String) -> Vec<T>
where
    T: std::str::FromStr,
    T: std::fmt::Debug,
    T::Err: std::fmt::Debug,
    T::Err: std::fmt::Display,
{
    let (elements, errors): (Vec<_>, Vec<_>) = read::<String>(&line)
        .split_ascii_whitespace()
        .map(|e| e.parse())
        .partition(Result::is_ok);
    if errors.len() > 0 {
        panic!("Error parsing line with vec {line}, errors: {errors:?}");
    }
    elements.into_iter().map(Result::unwrap).collect()
}

fn read_tuple<T: std::str::FromStr, S: std::str::FromStr>(line: String) -> (T, S) {
    let line = read::<String>(&line);
    let mut iter = line.split_ascii_whitespace();
    let t = iter
        .next()
        .expect("Error parsing line with tuples {}")
        .parse();
    let s = iter
        .next()
        .expect("Error parsing line with tuples {}")
        .parse();
    match (t, s) {
        (Ok(t), Ok(s)) => (t, s),
        _ => panic!("Error parsing line with tuples {line}"),
    }
}

macro_rules! parse_vec {
    ($name: ident, $type: ident) => {
        #[gen_stub_pyfunction]
        #[pyfunction]
        pub fn $name(file_path: &str) -> Vec<Vec<$type>> {
            read_lines(file_path).iter().map(read_vec).collect()
        }
    };
}

parse_vec!(file_parse_vec_i32, i32);
parse_vec!(file_parse_vec_bool, bool);
parse_vec!(file_parse_vec_char, char);
parse_vec!(file_parse_vec_f64, f64);
