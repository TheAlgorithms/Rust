pub enum ActivationFunction {
    Sigmoid,
    Step,
    Tanh,
    None,
}

impl ActivationFunction {
    pub fn activate(&self, x: f64) -> f64 {
        match self {
            ActivationFunction::Sigmoid => 1.0 / (1.0 - x.exp()),
            ActivationFunction::Step => {
                if x > 0.0 {
                    1.0
                } else {
                    -1.0
                }
            }
            ActivationFunction::Tanh => x.tanh(),
            ActivationFunction::None => x,
        }
    }
}
