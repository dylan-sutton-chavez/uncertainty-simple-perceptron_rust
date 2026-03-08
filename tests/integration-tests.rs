use rstest::*;
use serde::Deserialize;

use perceptron::model;

#[derive(Deserialize)]
struct TestCase {

    /*
    Defines the data structure for each test case.
    */

    input: Vec<f32>,
    expected: i32

}

#[fixture]
fn cases() -> Vec<TestCase> {

    /*
    Deserialize the tests cases to a vector whit each case. 
    */

    let json: &str = include_str!("cases.json");
    
    serde_json::from_str(json).unwrap()

}

#[rstest]
fn test_inference(cases: Vec<TestCase>) {

    /*
    Run the tests cases whit a loaded model.
    */

    let mut perceptron = model::Perceptron::new();

    perceptron.weights = vec![0.5, -0.2];
    perceptron.bias = 0.1;

    for case in cases {

        assert_eq!(perceptron.inference(case.input), case.expected); // Run each case and mirror whit the expected result.
    
    }

}