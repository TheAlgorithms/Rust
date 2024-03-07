// Enum representing different activation functions for the perceptron model.
pub enum ActivationFunction {
    Sigmoid,  // Sigmoid activation function
    Step,     // Step (or Heaviside) activation function
    Tanh,     // Hyperbolic tangent activation function
    None,     // No activation function (identity)
}

impl ActivationFunction {
    // Activate function computes the output of the activation function given an input value.
    pub fn activate(&self, x: f64) -> f64 {
        match self {
            ActivationFunction::Sigmoid => 1.0 / (1.0 + (-x).exp()),  // Sigmoid: 1 / (1 + e^(-x))
            ActivationFunction::Step => {                            // Step (Heaviside): 1 if x > 0, -1 otherwise
                if x > 0.0 {
                    1.0
                } else {
                    -1.0
                }
            }
            ActivationFunction::Tanh => x.tanh(),                    // Tanh: Hyperbolic tangent function
            ActivationFunction::None => x,                           
        }
    }
}
