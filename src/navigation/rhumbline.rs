use std::f64::consts::PI;

const EARTH_RADIUS: f64 = 6371000.0;

pub fn rhumb_dist(lat1: f64, long1: f64, lat2: f64, long2: f64) -> f64 {
    let phi1 = lat1 * PI / 180.00;
    let phi2 = lat2 * PI / 180.00;
    let del_phi = phi2 - phi1;
    let mut del_lambda = (long2 - long1) * PI / 180.00;

    if del_lambda > PI {
        del_lambda -= 2.00 * PI;
    } else if del_lambda < -PI {
        del_lambda += 2.00 * PI;
    }

    let del_psi = ((phi2 / 2.00 + PI / 4.00).tan() / (phi1 / 2.00 + PI / 4.00).tan()).ln();
    let q = match del_psi.abs() > 1e-12 {
        true => del_phi / del_psi,
        false => phi1.cos(),
    };

    (del_phi.powf(2.00) + (q * del_lambda).powf(2.00)).sqrt() * EARTH_RADIUS
}

pub fn rhumb_bearing(lat1: f64, long1: f64, lat2: f64, long2: f64) -> f64 {
    let phi1 = lat1 * PI / 180.00;
    let phi2 = lat2 * PI / 180.00;
    let mut del_lambda = (long2 - long1) * PI / 180.00;

    if del_lambda > PI {
        del_lambda -= 2.00 * PI;
    } else if del_lambda < -PI {
        del_lambda += 2.00 * PI;
    }

    let del_psi = ((phi2 / 2.00 + PI / 4.00).tan() / (phi1 / 2.00 + PI / 4.00).tan()).ln();
    let bearing = del_lambda.atan2(del_psi) * 180.0 / PI;
    (bearing + 360.00) % 360.00
}
pub fn rhumb_destination(lat: f64, long: f64, distance: f64, bearing: f64) -> (f64, f64) {
    let del = distance / EARTH_RADIUS;
    let phi1 = lat * PI / 180.00;
    let lambda1 = long * PI / 180.00;
    let theta = bearing * PI / 180.00;

    let del_phi = del * theta.cos();
    let phi2 = (phi1 + del_phi).clamp(-PI / 2.0, PI / 2.0);

    let del_psi = ((phi2 / 2.00 + PI / 4.00).tan() / (phi1 / 2.0 + PI / 4.0).tan()).ln();
    let q = match del_psi.abs() > 1e-12 {
        true => del_phi / del_psi,
        false => phi1.cos(),
    };

    let del_lambda = del * theta.sin() / q;
    let lambda2 = lambda1 + del_lambda;

    (phi2 * 180.00 / PI, lambda2 * 180.00 / PI)
}

//TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rhumb_distance() {
        let distance = rhumb_dist(28.5416, 77.2006, 28.5457, 77.1928);
        assert!(distance > 700.00 && distance < 1000.0);
    }

    #[test]
    fn test_rhumb_bearing() {
        let bearing = rhumb_bearing(28.5416, 77.2006, 28.5457, 77.1928);
        assert!((bearing - 300.0).abs() < 5.0);
    }

    #[test]
    fn test_rhumb_destination_point() {
        let (lat, lng) = rhumb_destination(28.5457, 77.1928, 1000.00, 305.0);
        assert!((lat - 28.550).abs() < 0.010);
        assert!((lng - 77.1851).abs() < 0.010);
    }
}
