//! # AdamW (Adam with decoupled weight decay) optimizer
//!
//! AdamW modifies the standard Adam optimizer by decoupling weight decay from the
//! gradient update step. In standard Adam, weight decay is typically implemented
//! by adding an L2 penalty to the loss, which interacts with the adaptive learning
//! rates in a way that often results in suboptimal model convergence.
//!
//! AdamW explicitly decays the weights prior to the gradient update, restoring
//! the original mathematical definition of weight decay and generally enabling
//! better performance on complex models such as transformers.
//!
//! ## Resources:
//!   - Decoupled Weight Decay Regularization (by Ilya Loshchilov and Frank Hutter):
//!       - [https://arxiv.org/abs/1711.05101]
//!   - PyTorch AdamW optimizer:
//!       - [https://pytorch.org/docs/stable/generated/torch.optim.AdamW.html]

#[allow(dead_code)]
pub struct AdamW {
    learning_rate: f64, // alpha: initial step size
    betas: (f64, f64),  // betas: exponential decay rates for moment estimates
    epsilon: f64,       // epsilon: prevent division by zero
    weight_decay: f64,  // weight_decay: decouples weight decay penalty
    m: Vec<f64>,        // m: biased first moment estimate of gradient
    v: Vec<f64>,        // v: biased second raw moment estimate of gradient
    t: usize,           // t: time step
}

#[allow(dead_code)]
impl AdamW {
    pub fn new(
        learning_rate: Option<f64>,
        betas: Option<(f64, f64)>,
        epsilon: Option<f64>,
        weight_decay: Option<f64>,
        params_len: usize,
    ) -> Self {
        AdamW {
            learning_rate: learning_rate.unwrap_or(1e-3),
            betas: betas.unwrap_or((0.9, 0.999)),
            epsilon: epsilon.unwrap_or(1e-8),
            weight_decay: weight_decay.unwrap_or(1e-2), // default weight decay scaling
            m: vec![0.0; params_len],
            v: vec![0.0; params_len],
            t: 0,
        }
    }

    /// Computes the AdamW step, updating the model parameters directly inline to
    /// properly enable decoupled weight decay modifications.
    pub fn step(&mut self, params: &mut [f64], gradients: &[f64]) {
        assert_eq!(
            params.len(),
            gradients.len(),
            "Parameters and gradients must be identical sizes."
        );
        self.t += 1;

        for i in 0..gradients.len() {
            // Apply decoupled weight decay (the 'W' in AdamW) inline
            params[i] -= self.learning_rate * self.weight_decay * params[i];

            // update biased first and second moment estimate
            self.m[i] = self.betas.0 * self.m[i] + (1.0 - self.betas.0) * gradients[i];
            self.v[i] = self.betas.1 * self.v[i] + (1.0 - self.betas.1) * gradients[i].powi(2);

            // bias correction
            let m_hat = self.m[i] / (1.0 - self.betas.0.powi(self.t as i32));
            let v_hat = self.v[i] / (1.0 - self.betas.1.powi(self.t as i32));

            // Apply standard Adam adaptive learning rate step
            params[i] -= self.learning_rate * m_hat / (v_hat.sqrt() + self.epsilon);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adamw_init_default_values() {
        let optimizer = AdamW::new(None, None, None, None, 1);

        assert_eq!(optimizer.learning_rate, 0.001);
        assert_eq!(optimizer.betas, (0.9, 0.999));
        assert_eq!(optimizer.epsilon, 1e-8);
        assert_eq!(optimizer.weight_decay, 1e-2);
        assert_eq!(optimizer.m, vec![0.0; 1]);
        assert_eq!(optimizer.v, vec![0.0; 1]);
        assert_eq!(optimizer.t, 0);
    }

    #[test]
    fn test_adamw_init_custom_values() {
        let optimizer = AdamW::new(Some(0.1), Some((0.8, 0.888)), Some(1e-4), Some(0.005), 3);

        assert_eq!(optimizer.learning_rate, 0.1);
        assert_eq!(optimizer.betas, (0.8, 0.888));
        assert_eq!(optimizer.epsilon, 1e-4);
        assert_eq!(optimizer.weight_decay, 0.005);
        assert_eq!(optimizer.m, vec![0.0; 3]);
        assert_eq!(optimizer.v, vec![0.0; 3]);
        assert_eq!(optimizer.t, 0);
    }

    #[test]
    fn test_adamw_step_default_params() {
        let gradients = vec![-1.0, 2.0, -3.0];
        let mut params = vec![0.5, -0.5, 0.0]; // non-zero starting params to test wd

        let mut optimizer = AdamW::new(None, None, None, None, 3);
        optimizer.step(&mut params, &gradients);

        // Calculate expected values conceptually manually
        // For i=0 (val = 0.5, grad = -1.0)
        // param = 0.5 - (0.001 * 0.01 * 0.5) = 0.5 - 0.000005 = 0.499995
        // m = 0.9(0) + 0.1(-1.0) = -0.1
        // v = 0.999(0) + 0.001(1.0) = 0.001
        // m_hat = -0.1 / 0.1 = -1.0
        // v_hat = 0.001 / 0.001 = 1.0
        // param -= 0.001 * -1.0 / (1.0 + 1e-8)
        // final param roughly 0.499995 + 0.001 = 0.50099499999
        assert!(params[0] > 0.5);
        assert!(params[1] < -0.5);
    }

    #[test]
    fn test_adamw_step_zero_gradients_with_weight_decay() {
        // If gradients are zero, params should strictly decay toward zero.
        let gradients = vec![0.0, 0.0];
        let mut params = vec![100.0, -100.0];

        let mut optimizer = AdamW::new(Some(1.0), None, None, Some(0.1), 2); // 10% daily decay
        optimizer.step(&mut params, &gradients);

        assert_eq!(params, vec![90.0, -90.0]); // 10% toward 0
        optimizer.step(&mut params, &gradients);
        assert_eq!(params, vec![81.0, -81.0]);
    }

    #[ignore]
    #[test]
    fn test_adamw_step_iteratively_until_convergence() {
        const CONVERGENCE_THRESHOLD: f64 = 1e-4;
        let gradients = vec![1.0, 2.0, 3.0, 4.0];

        let mut optimizer = AdamW::new(Some(0.01), None, None, Some(1e-4), 4);
        let mut model_params = vec![5.0; 4];

        let mut updates_made = true;
        let mut loops = 0;

        while updates_made && loops < 1000 {
            let old_params = model_params.clone();
            optimizer.step(&mut model_params, &gradients);

            let mut diff = 0.0;
            for i in 0..model_params.len() {
                diff += (old_params[i] - model_params[i]).powi(2);
            }
            if diff.sqrt() < CONVERGENCE_THRESHOLD {
                updates_made = false;
            }
            loops += 1;
        }

        assert!(
            loops < 1000,
            "Optimizer failed to converge within 1000 epochs."
        );

        // Because the gradient is constantly pushing against it, AdamW will find an equilibrium point
        // balancing the gradient direction with the weight decay pressure.
        assert!(model_params[0] < 5.0);
    }
}
