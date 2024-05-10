use pyo3::pyfunction;



#[pyfunction]
pub fn run_knn_model(){
    prepare_data();
    train_model();
    test_model();
}



fn prepare_data(){
    println!("Hello from knn_model module")
}

fn train_model(){}


fn test_model(){}