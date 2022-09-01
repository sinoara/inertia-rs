use pyo3::prelude::*;
use pyo3::PyClass;

use crate::System;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

//impl PyClass for System{};

/// A Python module implemented in Rust.
#[pymodule]
fn inertia_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<System>()?;
    Ok(())
}
