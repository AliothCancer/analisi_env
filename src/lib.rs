pub mod iced_app;
pub mod plotter_svg_generator;

use iced_app::run_app;
use plotter_svg_generator::generate;
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn analisi_env(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(sum_numbers, m)?)?;
    m.add_function(wrap_pyfunction!(plot, m)?)?;
    Ok(())
}

#[pyfunction]
fn sum_numbers(number: u128) -> PyResult<u128> {
    let rng = 0..number;
    Ok(rng.into_iter().sum())
}

#[allow(unused)]
#[pyfunction]
fn plot(one_ddata: Vec<f32>) -> PyResult<()> {
    match generate() {
        Ok(_) => (),
        Err(e) => println!("{e:?}")
    };
    run_app();
    Ok(())
}
