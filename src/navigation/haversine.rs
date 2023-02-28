use std::f64::consts::PI;

const EARTH_RADIUS: f64 = 6371000.00;

pub fn haversine(lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
    let delta_dist_lat = (lat2 - lat1) * PI / 180.0;
    let delta_dist_lng = (lng2 - lng1) * PI / 180.0;

    let cos1 = lat1 * PI / 180.0;
    let cos2 = lat2 * PI / 180.0;

    let delta_lat = (delta_dist_lat / 2.0).sin().powf(2.0);
    let delta_lng = (delta_dist_lng / 2.0).sin().powf(2.0);

    let a = delta_lat + delta_lng * cos1.cos() * cos2.cos();
    let result = 2.0 * a.asin().sqrt();

    result * EARTH_RADIUS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        assert_eq!(
            format!(
                "{:.2}km",
                haversine(52.375603, 4.903206, 52.366059, 4.926692) / 1000.0
            ),
            "1.92km"
        );
    }
}
