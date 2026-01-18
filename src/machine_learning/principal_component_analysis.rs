/// Principal Component Analysis (PCA) for dimensionality reduction.
/// PCA transforms data to a new coordinate system where the greatest
/// variance lies on the first coordinate (first principal component),
/// the second greatest variance on the second coordinate, and so on.

/// Compute the mean of each feature across all samples
fn compute_means(data: &[Vec<f64>]) -> Vec<f64> {
    if data.is_empty() {
        return vec![];
    }

    let num_features = data[0].len();
    let mut means = vec![0.0; num_features];

    for sample in data {
        for (i, &feature) in sample.iter().enumerate() {
            means[i] += feature;
        }
    }

    let n = data.len() as f64;
    for mean in &mut means {
        *mean /= n;
    }

    means
}

/// Center the data by subtracting the mean from each feature
fn center_data(data: &[Vec<f64>], means: &[f64]) -> Vec<Vec<f64>> {
    data.iter()
        .map(|sample| {
            sample
                .iter()
                .zip(means.iter())
                .map(|(&x, &mean)| x - mean)
                .collect()
        })
        .collect()
}

/// Compute covariance matrix from centered data
fn compute_covariance_matrix(centered_data: &[Vec<f64>]) -> Vec<f64> {
    if centered_data.is_empty() {
        return vec![];
    }

    let n = centered_data.len();
    let num_features = centered_data[0].len();

    let mut cov_matrix = vec![0.0; num_features * num_features];

    for i in 0..num_features {
        for j in i..num_features {
            let mut cov = 0.0;
            for sample in centered_data {
                cov += sample[i] * sample[j];
            }
            cov /= n as f64;

            cov_matrix[i * num_features + j] = cov;
            cov_matrix[j * num_features + i] = cov;
        }
    }

    cov_matrix
}

/// Power iteration method to find the dominant eigenvalue and eigenvector
fn power_iteration(matrix: &[f64], n: usize, max_iter: usize, tolerance: f64) -> (f64, Vec<f64>) {
    let mut b_k = vec![1.0; n];
    let mut b_k_prev = vec![0.0; n];

    for _ in 0..max_iter {
        b_k_prev.clone_from(&b_k);

        let mut b_k_new = vec![0.0; n];
        for i in 0..n {
            for j in 0..n {
                b_k_new[i] += matrix[i * n + j] * b_k[j];
            }
        }

        let norm = b_k_new.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm > 1e-10 {
            for val in &mut b_k_new {
                *val /= norm;
            }
        }

        b_k = b_k_new;

        let diff: f64 = b_k
            .iter()
            .zip(b_k_prev.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(0.0, |acc, x| acc.max(x));

        if diff < tolerance {
            break;
        }
    }

    let eigenvalue = b_k
        .iter()
        .enumerate()
        .map(|(i, &val)| {
            let mut row_sum = 0.0;
            for j in 0..n {
                row_sum += matrix[i * n + j] * b_k[j];
            }
            row_sum * val
        })
        .sum::<f64>()
        / b_k.iter().map(|x| x * x).sum::<f64>();

    (eigenvalue, b_k)
}

/// Deflate a matrix by removing the component along a given eigenvector
fn deflate_matrix(matrix: &[f64], eigenvector: &[f64], eigenvalue: f64, n: usize) -> Vec<f64> {
    let mut deflated = matrix.to_vec();

    for i in 0..n {
        for j in 0..n {
            deflated[i * n + j] -= eigenvalue * eigenvector[i] * eigenvector[j];
        }
    }

    deflated
}

/// Perform PCA on the input data
/// Returns transformed data with reduced dimensions
pub fn principal_component_analysis(
    data: Vec<Vec<f64>>,
    num_components: usize,
) -> Option<Vec<Vec<f64>>> {
    if data.is_empty() {
        return None;
    }

    let num_features = data[0].len();

    if num_features == 0 {
        return None;
    }

    if num_components > num_features {
        return None;
    }

    if num_components == 0 {
        return None;
    }

    let means = compute_means(&data);
    let centered_data = center_data(&data, &means);
    let cov_matrix = compute_covariance_matrix(&centered_data);

    let mut eigenvectors = Vec::new();
    let mut deflated_matrix = cov_matrix;

    for _ in 0..num_components {
        let (_eigenvalue, eigenvector) =
            power_iteration(&deflated_matrix, num_features, 1000, 1e-10);
        eigenvectors.push(eigenvector);
        deflated_matrix = deflate_matrix(
            &deflated_matrix,
            eigenvectors.last().unwrap(),
            _eigenvalue,
            num_features,
        );
    }

    let transformed_data: Vec<Vec<f64>> = centered_data
        .iter()
        .map(|sample| {
            (0..num_components)
                .map(|k| {
                    eigenvectors[k]
                        .iter()
                        .zip(sample.iter())
                        .map(|(&ev, &s)| ev * s)
                        .sum::<f64>()
                })
                .collect()
        })
        .collect();

    Some(transformed_data)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pca_simple() {
        let data = vec![
            vec![1.0, 2.0],
            vec![2.0, 3.0],
            vec![3.0, 4.0],
            vec![4.0, 5.0],
            vec![5.0, 6.0],
        ];

        let result = principal_component_analysis(data, 1);
        assert!(result.is_some());

        let transformed = result.unwrap();
        assert_eq!(transformed.len(), 5);
        assert_eq!(transformed[0].len(), 1);

        let all_values: Vec<f64> = transformed.iter().map(|v| v[0]).collect();
        let mean = all_values.iter().sum::<f64>() / all_values.len() as f64;

        assert!((mean).abs() < 1e-5);
    }

    #[test]
    fn test_pca_empty_data() {
        let data = vec![];
        let result = principal_component_analysis(data, 2);
        assert_eq!(result, None);
    }

    #[test]
    fn test_pca_empty_features() {
        let data = vec![vec![], vec![]];
        let result = principal_component_analysis(data, 1);
        assert_eq!(result, None);
    }

    #[test]
    fn test_pca_invalid_num_components() {
        let data = vec![vec![1.0, 2.0], vec![2.0, 3.0]];

        let result = principal_component_analysis(data.clone(), 3);
        assert_eq!(result, None);

        let result = principal_component_analysis(data, 0);
        assert_eq!(result, None);
    }

    #[test]
    fn test_pca_preserves_dimensions() {
        let data = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ];

        let result = principal_component_analysis(data, 2);
        assert!(result.is_some());

        let transformed = result.unwrap();
        assert_eq!(transformed.len(), 3);
        assert_eq!(transformed[0].len(), 2);
    }

    #[test]
    fn test_pca_reconstruction_variance() {
        let data = vec![
            vec![2.5, 2.4],
            vec![0.5, 0.7],
            vec![2.2, 2.9],
            vec![1.9, 2.2],
            vec![3.1, 3.0],
            vec![2.3, 2.7],
            vec![2.0, 1.6],
            vec![1.0, 1.1],
            vec![1.5, 1.6],
            vec![1.1, 0.9],
        ];

        let result = principal_component_analysis(data, 1);
        assert!(result.is_some());

        let transformed = result.unwrap();
        assert_eq!(transformed.len(), 10);
        assert_eq!(transformed[0].len(), 1);
    }

    #[test]
    fn test_center_data() {
        let data = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ];

        let means = vec![4.0, 5.0, 6.0];
        let centered = center_data(&data, &means);

        assert_eq!(centered[0], vec![-3.0, -3.0, -3.0]);
        assert_eq!(centered[1], vec![0.0, 0.0, 0.0]);
        assert_eq!(centered[2], vec![3.0, 3.0, 3.0]);
    }

    #[test]
    fn test_compute_means() {
        let data = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ];

        let means = compute_means(&data);
        assert_eq!(means, vec![4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_power_iteration() {
        let matrix = vec![4.0, 1.0, 1.0, 1.0, 3.0, 1.0, 1.0, 1.0, 2.0];

        let (eigenvalue, eigenvector) = power_iteration(&matrix, 3, 1000, 1e-10);

        assert!(eigenvalue > 0.0);
        assert_eq!(eigenvector.len(), 3);

        let norm = eigenvector.iter().map(|x| x * x).sum::<f64>().sqrt();
        assert!((norm - 1.0).abs() < 1e-6);
    }
}
