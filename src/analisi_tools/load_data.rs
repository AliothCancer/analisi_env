


use polars::prelude::*;
use pyo3::pyfunction;


const DATA_FILE: &str = "data/data_usable_head.csv";


#[pyfunction]
pub fn data_head() -> (){
    
    let df = CsvReader::from_path(DATA_FILE)
            .unwrap()
            .has_header(true)
            .finish()
            .unwrap();
    
    let head = df.head(Some(5));


    println!("{}", head);
    
            
}