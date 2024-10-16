// Constants for WGS84 Earth ellipsoid model
const WGS84_A: f64 = 6378137.0;  // Semi-major axis in meters
const WGS84_B: f64 = 6356752.314245;  // Semi-minor axis in meters
const WGS84_F: f64 = 1.0 / 298.257223563;  // Flattening

/// Calculate the geodesic distance between two points on Earth using the Vincenty formula
///
/// # Arguments
/// * `lat1` - Latitude of the first point in degrees
/// * `lon1` - Longitude of the first point in degrees
/// * `lat2` - Latitude of the second point in degrees
/// * `lon2` - Longitude of the second point in degrees
///
/// # Returns
/// The distance in meters
pub fn vincenty(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    // Convert latitudes to radians and adjust for the ellipsoid's flattening
    let reduced_lat1 = (1.0 - WGS84_F) * lat1.to_radians().tan();
    let reduced_lat2 = (1.0 - WGS84_F) * lat2.to_radians().tan();
    let delta_longitude = (lon2 - lon1).to_radians();

    // Trigonometric precomputations
    let sin_lat1 = reduced_lat1.sin();
    let cos_lat1 = reduced_lat1.cos();
    let sin_lat2 = reduced_lat2.sin();
    let cos_lat2 = reduced_lat2.cos();

    let mut lambda = delta_longitude;
    let mut lambda_prev;
    let mut iterations = 100;

    let mut sin_sigma;
    let mut cos_sigma;
    let mut cos_sq_alpha;
    let mut cos2_sigma_m;
    let mut sigma = 0.0;

    // Iterate until convergence or the limit is reached
    while iterations > 0 {
        lambda_prev = lambda;

        let sin_lambda = lambda.sin();
        let cos_lambda = lambda.cos();

        sin_sigma = ((cos_lat2 * sin_lambda).powi(2) + 
                    (cos_lat1 * sin_lat2 - sin_lat1 * cos_lat2 * cos_lambda).powi(2)).sqrt();
        if sin_sigma == 0.0 {
            return 0.0; // Points are coincident
        }

        cos_sigma = sin_lat1 * sin_lat2 + cos_lat1 * cos_lat2 * cos_lambda;
        sigma = sin_sigma.atan2(cos_sigma);

        let sin_alpha = cos_lat1 * cos_lat2 * sin_lambda / sin_sigma;
        cos_sq_alpha = 1.0 - sin_alpha.powi(2);

        cos2_sigma_m = if cos_sq_alpha == 0.0 {
            0.0 // Equatorial line
        } else {
            cos_sigma - 2.0 * sin_lat1 * sin_lat2 / cos_sq_alpha
        };

        let c = WGS84_F / 16.0 * cos_sq_alpha * (4.0 + WGS84_F * (4.0 - 3.0 * cos_sq_alpha));
        lambda = delta_longitude + (1.0 - c) * WGS84_F * sin_alpha * (sigma + c * sin_sigma * 
                (cos2_sigma_m + c * cos_sigma * (-1.0 + 2.0 * cos2_sigma_m.powi(2))));

        if (lambda - lambda_prev).abs() < 1e-12 {
            break;  // Convergence reached
        }
        iterations -= 1;
    }

    if iterations == 0 {
        return f64::NAN; // Formula failed to converge
    }

    // Final calculations for the distance
    let u_sq = cos_sq_alpha * (WGS84_A.powi(2) - WGS84_B.powi(2)) / WGS84_B.powi(2);
    let a = 1.0 + u_sq / 16384.0 * (4096.0 + u_sq * (-768.0 + u_sq * (320.0 - 175.0 * u_sq)));
    let b = u_sq / 1024.0 * (256.0 + u_sq * (-128.0 + u_sq * (74.0 - 47.0 * u_sq)));

    let delta_sigma = b * sin_sigma * (cos2_sigma_m + b / 4.0 * (cos_sigma * (-1.0 + 2.0 * cos2_sigma_m.powi(2)) - 
                     b / 6.0 * cos2_sigma_m * (-3.0 + 4.0 * sin_sigma.powi(2)) * (-3.0 + 4.0 * cos2_sigma_m.powi(2))));

    let distance = WGS84_B * a * (sigma - delta_sigma);

    distance // distance in meters
}
