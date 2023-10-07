//Rust implementation of the Sigmoid activation function.
//The formula for Sigmoid: 1 / (1 + e^(-x))
//More information on the concepts of Sigmoid can be found here:
//https://en.wikipedia.org/wiki/Sigmoid_function

//The function below takes a reference to a mutable <f32> Vector as an argument
//and returns the vector with 'Sigmoid' applied to all values.
//Of course, these functions can be changed by the developer so that the input vector isn't manipulated.
//This is simply an implemenation of the formula.

use std::f32::consts::E;

pub fn sigmoid(array: &mut Vec<f32>) -> &mut Vec<f32> {
    //note that these calculations are assuming the Vector values consists of real numbers in radians
    for value in &mut *array {
        *value = 1. / (1. + E.powf(-1. * *value));
    }

    array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sigmoid() {
        let mut test = Vec::from([1.0, 0.5, -1.0, 0.0, 0.3]);
        assert_eq!(
            sigmoid(&mut test),
            &mut Vec::<f32>::from([0.7310586, 0.62245935, 0.26894143, 0.5, 0.5744425,])
        );
    }
}
