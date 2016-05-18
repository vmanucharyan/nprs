extern crate rusty_machine;

use rusty_machine::learning::logistic_reg::LogisticRegressor;
use rusty_machine::learning::SupModel;
use rusty_machine::linalg::matrix::Matrix;
use rusty_machine::linalg::vector::Vector;

fn main() {
    let inputs = Matrix::new(4,1,vec![1.0,3.0,5.0,7.0]);
    let targets = Vector::new(vec![0.,0.,1.,1.]);

    let mut log_mod = LogisticRegressor::default();

    // Train the model
    log_mod.train(&inputs, &targets);

    // Now we'll predict a new point
    let new_point = Matrix::new(1,1,vec![110.]);
    let output = log_mod.predict(&new_point);

    println!("{:?}", &output);
}
