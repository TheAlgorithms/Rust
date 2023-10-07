//Rust implementation of the Tanh (hyperbolic tangent) activation function.
//The formula for Tanh: (e^x - e^(-x))/(e^x + e^(-x)) OR (2/(1+e^(-2x))-1
//More information on the concepts of Sigmoid can be found here:
//https://en.wikipedia.org/wiki/Hyperbolic_functions

//The function below takes a reference to a mutable <f32> Vector as an argument
//and returns the vector with 'Tanh' applied to all values.
//Of course, these functions can be changed by the developer so that the input vector isn't manipulated.
//This is simply an implemenation of the formula.

use std::f32::consts::E;

pub fn tanh(array: &mut Vec<f32>) -> &mut Vec<f32> {
    //note that these calculations are assuming the Vector values consists of real numbers in radians
    for value in &mut *array {
        *value = (2. / (1. + E.powf(-2. * *value))) - 1.;
    }

    array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tanh() {
        let mut test = Vec::from([1.0, 0.5, -1.0, 0.0, 0.3]);
        assert_eq!(
            tanh(&mut test),
            &mut Vec::<f32>::from([0.76159406, 0.4621172, -0.7615941, 0.0, 0.29131258,])
        );
    }
}
