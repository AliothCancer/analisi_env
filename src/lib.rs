pub mod plot_things;
pub mod analisi_tools;

use plot_things::iced_app::run_app;
use plot_things::plotter_svg_generator::generate;
use analisi_tools::load_data;
use pyo3::prelude::*;


/// A Python module implemented in Rust.
#[pymodule]
fn analisi_env(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(plot, m)?)?;
    m.add_function(wrap_pyfunction!(load_data::data_head,m)?)?;
    Ok(())
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
