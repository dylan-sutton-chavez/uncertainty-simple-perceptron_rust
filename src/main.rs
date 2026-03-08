use log::{info, LevelFilter};
use simple_logger::SimpleLogger;

fn setup() {

    /*
    Setup the needed dependencies for the program.
    */

    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .expect("An error ocurred on the initialization of logging system.");

    info!("Logging system initialized correctly.");

}

mod model;

fn main() {

    /*
    Starting point for the application.
    */
    
    setup();

    let mut perceptron = model::Perceptron::new();
    
    perceptron.weights = vec![0.5, -0.2];
    perceptron.bias = 0.1;
    
    let values = vec![1.0, 2.0];
    let model_pred = perceptron.inference(values);
    
    info!("Perceptron result: {model_pred}");
    
}