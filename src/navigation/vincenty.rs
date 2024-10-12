// vincenty.rs
const WGS84_A: f64 = 6378137.0; // Semi-major axis in meters
const WGS84_B: f64 = 6356752.314245; // Semi-minor axis in meters
const WGS84_F: f64 = 1.0 / 298.257223563; // Flattening

pub fn vincenty(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let u1 = (1.0 - WGS84_F) * lat1.to_radians().tan();
    let u2 = (1.0 - WGS84_F) * lat2.to_radians().tan();
    let l = (lon2 - lon1).to_radians();
    
    let sin_u1 = u1.sin();
    let cos_u1 = u1.cos();
    let sin_u2 = u2.sin();
    let cos_u2 = u2.cos();
    
    let lambda = l;
    let (sin_lambda, cos_lambda) = (lambda.sin(), lambda.cos());
    
    let mut sin_sigma;
    let mut cos_sigma;
    let mut cos_sq_alpha;
    let mut cos2_sigma_m;
    let mut sigma = 0.0;
    
    let mut iteration_limit = 100;
    let mut lambda_prev = 0.0;
    
    while (lambda - lambda_prev).abs() > 1e-12 && iteration_limit > 0 {
        let sin_lambda = lambda.sin();
        let cos_lambda = lambda.cos();
        
        sin_sigma = ((cos_u2 * sin_lambda).powi(2) +
                     (cos_u1 * sin_u2 - sin_u1 * cos_u2 * cos_lambda).powi(2)).sqrt();
        
        if sin_sigma == 0.0 {
            return 0.0; // coincident points
        }
        
        cos_sigma = sin_u1 * sin_u2 + cos_u1 * cos_u2 * cos_lambda;
        sigma = sin_sigma.atan2(cos_sigma);
        
        let sin_alpha = cos_u1 * cos_u2 * sin_lambda;
        cos_sq_alpha = 1.0 - sin_alpha.powi(2);
        
        cos2_sigma_m = if cos_sq_alpha == 0.0 {
            0.0
        } else {
            cos_u2 * cos_u1 * cos_lambda / cos_sq_alpha
        };
        
        let c = WGS84_F / 16.0 * cos_sq_alpha * (4.0 + WGS84_F * (4.0 - 3.0 * cos_sq_alpha));
        
        lambda_prev = lambda;
        lambda = l + (1.0 - c) * WGS84_F * sin_alpha * (sigma + c * sin_sigma * (cos2_sigma_m + c * cos_sigma * (-1.0 + 2.0 * cos2_sigma_m.powi(2))));
        
        iteration_limit -= 1;
    }
    
    if iteration_limit == 0 {
        return f64::NAN; // formula failed to converge
    }
    
    let u_sq = cos_sq_alpha * (WGS84_A.powi(2) - WGS84_B.powi(2)) / WGS84_B.powi(2);
    let a = 1.0 + u_sq / 16384.0 * (4096.0 + u_sq * (-768.0 + u_sq * (320.0 - 175.0 * u_sq)));
    let b = u_sq / 1024.0 * (256.0 + u_sq * (-128.0 + u_sq * (74.0 - 47.0 * u_sq)));
    
    let delta_sigma = b * sin_sigma * (cos2_sigma_m + b / 4.0 * (cos_sigma * (-1.0 + 2.0 * cos2_sigma_m.powi(2)) - 
                      b / 6.0 * cos2_sigma_m * (-3.0 + 4.0 * sin_sigma.powi(2)) * (-3.0 + 4.0 * cos2_sigma_m.powi(2))));
    
    let s = WGS84_B * a * (sigma - delta_sigma);
    
    s // distance in meters
}
