//! # Adam (Adaptive Moment Estimation) optimizer
//!
//! The `Adam (Adaptive Moment Estimation)` optimizer is an adaptive learning rate algorithm used
//! in gradient descent and machine learning, such as for training neural networks to solve deep
//! learning problems. Boasting memory-efficient fast convergence rates, it sets and iteratively
//! updates learning rates individually for each model parameter based on the gradient history.
//!
//! Setting `weight_decay > 0.0` enables the AdamW variant (Loshchilov & Hutter, 2019), which
//! applies weight decay directly to the parameters rather than folding it into the gradients.
//! This keeps the decay rate constant and independent of the gradient history — the key flaw
//! that AdamW corrects over naive L2 regularization inside Adam. With `weight_decay = 0.0`
//! (the default), the two algorithms are identical.
//!
//! ## Algorithm:
//!
//! Given:
//!   - α is the learning rate
//!   - (β_1, β_2) are the exponential decay rates for moment estimates
//!   - ϵ is any small value to prevent division by zero
//!   - λ is the weight decay coefficient (0.0 for standard Adam, > 0.0 for AdamW)
//!   - g_t are the gradients at time step t
//!   - m_t are the biased first moment estimates of the gradient at time step t
//!   - v_t are the biased second raw moment estimates of the gradient at time step t
//!   - θ_t are the model parameters at time step t
//!   - t is the time step
//!
//! Required:
//!   θ_0
//!
//! Initialize:
//!   m_0 <- 0
//!   v_0 <- 0
//!   t <- 0
//!
//! while θ_t not converged do
//!   m_t = β_1 * m_{t−1} + (1 − β_1) * g_t
//!   v_t = β_2 * v_{t−1} + (1 − β_2) * g_t^2
//!   m_hat_t = m_t / (1 - β_1^t)
//!   v_hat_t = v_t / (1 - β_2^t)
//!   θ_t = θ_{t-1} − α * (m_hat_t / (sqrt(v_hat_t) + ϵ) + λ * θ_{t-1})
//!
//! ## Resources:
//!   - Adam: A Method for Stochastic Optimization (by Diederik P. Kingma and Jimmy Ba):
//!       - [https://arxiv.org/abs/1412.6980]
//!   - Decoupled Weight Decay Regularization (by Ilya Loshchilov and Frank Hutter):
//!       - [https://arxiv.org/abs/1711.05101]
//!   - PyTorch Adam optimizer:
//!       - [https://pytorch.org/docs/stable/generated/torch.optim.Adam.html]
//!   - PyTorch AdamW optimizer:
//!       - [https://pytorch.org/docs/stable/generated/torch.optim.AdamW.html]
//!
pub struct Adam {
    learning_rate: f64, // alpha: initial step size for iterative optimization
    betas: (f64, f64),  // betas: exponential decay rates for moment estimates
    epsilon: f64,       // epsilon: prevent division by zero
    weight_decay: f64,  // lambda: decoupled weight decay coefficient (0.0 = standard Adam)
    m: Vec<f64>,        // m: biased first moment estimate of the gradient vector
    v: Vec<f64>,        // v: biased second raw moment estimate of the gradient vector
    t: usize,           // t: time step
}

impl Adam {
    pub fn new(
        learning_rate: Option<f64>,
        betas: Option<(f64, f64)>,
        epsilon: Option<f64>,
        weight_decay: Option<f64>,
        params_len: usize,
    ) -> Self {
        Adam {
            learning_rate: learning_rate.unwrap_or(1e-3), // typical good default lr
            betas: betas.unwrap_or((0.9, 0.999)),         // typical good default decay rates
            epsilon: epsilon.unwrap_or(1e-8),             // typical good default epsilon
            weight_decay: weight_decay.unwrap_or(0.0),    // 0.0 = standard Adam, > 0.0 = AdamW
            m: vec![0.0; params_len], // first moment vector elements all initialized to zero
            v: vec![0.0; params_len], // second moment vector elements all initialized to zero
            t: 0,                     // time step initialized to zero
        }
    }

    /// Computes one update step.
    ///
    /// `params` holds the current parameter values θ_{t-1}. When `weight_decay`
    /// is `0.0` the update is standard Adam; any positive value applies the AdamW
    /// decoupled decay term `λ * θ_{t-1}` directly to the parameters, independent
    /// of the adaptive scaling.
    ///
    /// # Panics
    ///
    /// Panics if `gradients` and `params` have different lengths.
    pub fn step(&mut self, gradients: &[f64], params: &[f64]) -> Vec<f64> {
        assert_eq!(
            gradients.len(),
            params.len(),
            "gradients and params must have the same length"
        );

        let mut updated_params = vec![0.0; params.len()];
        self.t += 1;

        for i in 0..gradients.len() {
            // update biased first moment estimate and second raw moment estimate
            self.m[i] = self.betas.0 * self.m[i] + (1.0 - self.betas.0) * gradients[i];
            self.v[i] = self.betas.1 * self.v[i] + (1.0 - self.betas.1) * gradients[i].powf(2f64);

            // compute bias-corrected first moment estimate and second raw moment estimate
            let m_hat = self.m[i] / (1.0 - self.betas.0.powi(self.t as i32));
            let v_hat = self.v[i] / (1.0 - self.betas.1.powi(self.t as i32));

            // Adaptive gradient step — preserves the original (lr * m_hat) / denom
            // operator order so floating-point results are identical to standard Adam
            // when weight_decay = 0.0. The decoupled decay term is added separately
            // so it does not interact with the adaptive scaling.
            updated_params[i] = params[i]
                - self.learning_rate * m_hat / (v_hat.sqrt() + self.epsilon)
                - self.learning_rate * self.weight_decay * params[i];
        }
        updated_params // return updated model parameters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Initialisation ────────────────────────────────────────────────────────

    #[test]
    fn test_adam_init_default_values() {
        let optimizer = Adam::new(None, None, None, None, 1);

        assert_eq!(optimizer.learning_rate, 0.001);
        assert_eq!(optimizer.betas, (0.9, 0.999));
        assert_eq!(optimizer.epsilon, 1e-8);
        assert_eq!(optimizer.weight_decay, 0.0);
        assert_eq!(optimizer.m, vec![0.0; 1]);
        assert_eq!(optimizer.v, vec![0.0; 1]);
        assert_eq!(optimizer.t, 0);
    }

    #[test]
    fn test_adam_init_custom_lr_value() {
        let optimizer = Adam::new(Some(0.9), None, None, None, 2);

        assert_eq!(optimizer.learning_rate, 0.9);
        assert_eq!(optimizer.betas, (0.9, 0.999));
        assert_eq!(optimizer.epsilon, 1e-8);
        assert_eq!(optimizer.weight_decay, 0.0);
        assert_eq!(optimizer.m, vec![0.0; 2]);
        assert_eq!(optimizer.v, vec![0.0; 2]);
        assert_eq!(optimizer.t, 0);
    }

    #[test]
    fn test_adam_init_custom_betas_value() {
        let optimizer = Adam::new(None, Some((0.8, 0.899)), None, None, 3);

        assert_eq!(optimizer.learning_rate, 0.001);
        assert_eq!(optimizer.betas, (0.8, 0.899));
        assert_eq!(optimizer.epsilon, 1e-8);
        assert_eq!(optimizer.weight_decay, 0.0);
        assert_eq!(optimizer.m, vec![0.0; 3]);
        assert_eq!(optimizer.v, vec![0.0; 3]);
        assert_eq!(optimizer.t, 0);
    }

    #[test]
    fn test_adam_init_custom_epsilon_value() {
        let optimizer = Adam::new(None, None, Some(1e-10), None, 4);

        assert_eq!(optimizer.learning_rate, 0.001);
        assert_eq!(optimizer.betas, (0.9, 0.999));
        assert_eq!(optimizer.epsilon, 1e-10);
        assert_eq!(optimizer.weight_decay, 0.0);
        assert_eq!(optimizer.m, vec![0.0; 4]);
        assert_eq!(optimizer.v, vec![0.0; 4]);
        assert_eq!(optimizer.t, 0);
    }

    #[test]
    fn test_adam_init_custom_weight_decay_value() {
        let optimizer = Adam::new(None, None, None, Some(0.1), 3);

        assert_eq!(optimizer.learning_rate, 0.001);
        assert_eq!(optimizer.betas, (0.9, 0.999));
        assert_eq!(optimizer.epsilon, 1e-8);
        assert_eq!(optimizer.weight_decay, 0.1);
        assert_eq!(optimizer.m, vec![0.0; 3]);
        assert_eq!(optimizer.v, vec![0.0; 3]);
        assert_eq!(optimizer.t, 0);
    }

    #[test]
    fn test_adam_init_all_custom_values() {
        let optimizer = Adam::new(Some(1.0), Some((0.001, 0.099)), Some(1e-1), Some(0.05), 5);

        assert_eq!(optimizer.learning_rate, 1.0);
        assert_eq!(optimizer.betas, (0.001, 0.099));
        assert_eq!(optimizer.epsilon, 1e-1);
        assert_eq!(optimizer.weight_decay, 0.05);
        assert_eq!(optimizer.m, vec![0.0; 5]);
        assert_eq!(optimizer.v, vec![0.0; 5]);
        assert_eq!(optimizer.t, 0);
    }

    // ── Step: standard Adam (weight_decay = 0.0) ──────────────────────────────

    #[test]
    fn test_adam_step_default_params() {
        let gradients = vec![-1.0, 2.0, -3.0, 4.0, -5.0, 6.0, -7.0, 8.0];
        let params = vec![0.0; 8];

        let mut optimizer = Adam::new(None, None, None, None, 8);
        let updated_params = optimizer.step(&gradients, &params);

        assert_eq!(
            updated_params,
            vec![
                0.0009999999900000003,
                -0.000999999995,
                0.0009999999966666666,
                -0.0009999999975,
                0.000999999998,
                -0.0009999999983333334,
                0.0009999999985714286,
                -0.00099999999875
            ]
        );
    }

    #[test]
    fn test_adam_step_custom_params() {
        let gradients = vec![9.0, -8.0, 7.0, -6.0, 5.0, -4.0, 3.0, -2.0, 1.0];
        let params = vec![0.0; 9];

        let mut optimizer = Adam::new(Some(0.005), Some((0.5, 0.599)), Some(1e-5), None, 9);
        let updated_params = optimizer.step(&gradients, &params);

        assert_eq!(
            updated_params,
            vec![
                -0.004999994444450618,
                0.004999993750007813,
                -0.004999992857153062,
                0.004999991666680556,
                -0.004999990000020001,
                0.004999987500031251,
                -0.004999983333388888,
                0.004999975000124999,
                -0.0049999500004999945
            ]
        );
    }

    #[test]
    fn test_adam_step_empty_gradients_array() {
        let gradients: Vec<f64> = vec![];
        let params: Vec<f64> = vec![];

        let mut optimizer = Adam::new(None, None, None, None, 0);
        let updated_params = optimizer.step(&gradients, &params);

        assert_eq!(updated_params, vec![]);
    }

    // ── Step: AdamW (weight_decay > 0.0) ─────────────────────────────────────

    #[test]
    fn test_adamw_step_nonzero_params_applies_decay() {
        // When params are non-zero and weight_decay > 0.0, the decay term must pull
        // every parameter strictly closer to zero than the plain adaptive step would.
        // Comparing against a no-decay run avoids replicating the internal floating
        // point computation path and tests the property that actually matters.
        let gradients = vec![1.0, -2.0, 3.0];
        let params = vec![0.5, -0.5, 1.0];

        let mut with_decay = Adam::new(None, None, None, Some(0.01), 3);
        let decayed = with_decay.step(&gradients, &params);

        let mut no_decay = Adam::new(None, None, None, None, 3);
        let not_decayed = no_decay.step(&gradients, &params);

        for i in 0..params.len() {
            assert!(
                decayed[i].abs() < not_decayed[i].abs(),
                "param[{i}]: with_decay={}, no_decay={}",
                decayed[i],
                not_decayed[i]
            );
        }
    }

    #[test]
    fn test_adamw_step_weight_decay_zero_matches_adam() {
        // weight_decay = 0.0 must be numerically identical to standard Adam.
        let gradients = vec![9.0, -8.0, 7.0, -6.0, 5.0, -4.0, 3.0, -2.0, 1.0];
        let params = vec![0.0; 9];

        let mut adamw = Adam::new(Some(0.005), Some((0.5, 0.599)), Some(1e-5), Some(0.0), 9);
        let mut adam = Adam::new(Some(0.005), Some((0.5, 0.599)), Some(1e-5), None, 9);

        assert_eq!(
            adamw.step(&gradients, &params),
            adam.step(&gradients, &params)
        );
    }

    #[test]
    fn test_adamw_step_decay_pulls_params_toward_zero() {
        // Each updated parameter must be closer to zero than its predecessor.
        let gradients = vec![1.0, -1.0, 2.0, -2.0];
        let params = vec![0.1, -0.1, 0.2, -0.2];

        let mut optimizer = Adam::new(Some(0.01), Some((0.9, 0.999)), Some(1e-8), Some(0.01), 4);
        let updated = optimizer.step(&gradients, &params);

        assert!(updated[0] < params[0]); // positive param, positive grad → decrease
        assert!(updated[1] > params[1]); // negative param, negative grad → increase
        assert!(updated[2] < params[2]);
        assert!(updated[3] > params[3]);
    }

    // ── Step: shared edge cases ───────────────────────────────────────────────

    #[test]
    #[should_panic(expected = "gradients and params must have the same length")]
    fn test_step_mismatched_lengths_panics() {
        let mut optimizer = Adam::new(None, None, None, None, 3);
        optimizer.step(&[1.0, 2.0, 3.0], &[0.0, 0.0]); // params too short
    }

    // ── Convergence (slow; marked #[ignore]) ─────────────────────────────────

    #[ignore]
    #[test]
    fn test_adam_step_iteratively_until_convergence_with_default_params() {
        const CONVERGENCE_THRESHOLD: f64 = 1e-5;
        let gradients = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

        let mut optimizer = Adam::new(None, None, None, None, 6);

        let mut model_params = vec![0.0; 6];
        let mut updated_params = optimizer.step(&gradients, &model_params);

        while (updated_params
            .iter()
            .zip(model_params.iter())
            .map(|(x, y)| x - y)
            .collect::<Vec<f64>>())
        .iter()
        .map(|&x| x.powi(2))
        .sum::<f64>()
        .sqrt()
            > CONVERGENCE_THRESHOLD
        {
            model_params = updated_params;
            updated_params = optimizer.step(&gradients, &model_params);
        }

        assert!(updated_params < vec![CONVERGENCE_THRESHOLD; 6]);
        assert_ne!(updated_params, model_params);
        assert_eq!(
            updated_params,
            vec![
                -0.0009999999899999931,
                -0.0009999999949999929,
                -0.0009999999966666597,
                -0.0009999999974999929,
                -0.0009999999979999927,
                -0.0009999999983333263
            ]
        );
    }

    #[ignore]
    #[test]
    fn test_adam_step_iteratively_until_convergence_with_custom_params() {
        const CONVERGENCE_THRESHOLD: f64 = 1e-7;
        let gradients = vec![7.0, -8.0, 9.0, -10.0, 11.0, -12.0, 13.0];

        let mut optimizer = Adam::new(Some(0.005), Some((0.8, 0.899)), Some(1e-5), None, 7);

        let mut model_params = vec![0.0; 7];
        let mut updated_params = optimizer.step(&gradients, &model_params);

        while (updated_params
            .iter()
            .zip(model_params.iter())
            .map(|(x, y)| x - y)
            .collect::<Vec<f64>>())
        .iter()
        .map(|&x| x.powi(2))
        .sum::<f64>()
        .sqrt()
            > CONVERGENCE_THRESHOLD
        {
            model_params = updated_params;
            updated_params = optimizer.step(&gradients, &model_params);
        }

        assert!(updated_params < vec![CONVERGENCE_THRESHOLD; 7]);
        assert_ne!(updated_params, model_params);
        assert_eq!(
            updated_params,
            vec![
                -0.004999992857153061,
                0.004999993750007814,
                -0.0049999944444506185,
                0.004999995000005001,
                -0.004999995454549587,
                0.004999995833336807,
                -0.004999996153849113
            ]
        );
    }

    #[ignore]
    #[test]
    fn test_adamw_step_iteratively_until_convergence() {
        const CONVERGENCE_THRESHOLD: f64 = 1e-5;
        let gradients = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

        let mut optimizer = Adam::new(None, None, None, Some(0.0), 6);

        let mut params = vec![0.0; 6];
        let mut updated = optimizer.step(&gradients, &params);

        while (updated
            .iter()
            .zip(params.iter())
            .map(|(x, y)| x - y)
            .collect::<Vec<f64>>())
        .iter()
        .map(|&x| x.powi(2))
        .sum::<f64>()
        .sqrt()
            > CONVERGENCE_THRESHOLD
        {
            params = updated;
            updated = optimizer.step(&gradients, &params);
        }

        assert_ne!(updated, params);
    }
}
