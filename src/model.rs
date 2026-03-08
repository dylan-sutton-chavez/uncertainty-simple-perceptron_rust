use log::{error};

pub struct Perceptron {

    /*
    Defines the data structure holding weights and bias.
    */

    pub weights: Vec<f32>,
    pub bias: f32

}

impl Perceptron { // Implementation block.

    pub fn new() -> Self{

        /*
        Initializes a new perceptron instance with default empty values.
        */

        Self {weights: vec![], bias: 0.0}

    }

    pub fn inference(&self, values: Vec<f32>) -> i32 {

        /*
        Run perceptron inference. 

        Args:
            values: &[f32] → Values vector.

        Out:
            i32 → Labeled output.
        */

        let model_pred: f32 = self.net_input(values);

        self.step_function(model_pred)

    }

    fn step_function(&self, model_pred: f32) -> i32 {
        
        /*
        Given a model pred, return the taged output.

        Args:
            model_pred: f32 → Model prediction value.

        Out:
            i32 → Binary label.
        */
        
        if model_pred >= 0.0 { 1 } else { 0 }

    }

    fn net_input (&self, values: Vec<f32>) -> f32 {

        /*
        Compute the dot product of the weights and values vectors, adding a bias term.

        Args:
            values: Vec<f32> → Values vector.

        Out:
            f32 → Net input.

        Maths:
            sum(w_i * x_i) + b
        */

        if values.len() != self.weights.len() {

            error!("The vector values and weights doesn't have the same length.");

            return 0.0

        }

        let mut cache: f32 = 0.0; // Can modify the variable.

        for i in 0..self.weights.len() { // Index range of the waights vector length.

            cache += self.weights[i] * values[i];

        }

        cache + self.bias

    }

}