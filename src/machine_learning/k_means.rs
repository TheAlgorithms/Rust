use rand::prelude::random;

fn get_distance(p1: &(f64, f64), p2: &(f64, f64)) -> f64 {
    let dx: f64 = p1.0 - p2.0;
    let dy: f64 = p1.1 - p2.1;

    ((dx * dx) + (dy * dy)).sqrt()
}

fn find_nearest(data_point: &(f64, f64), centroids: &[(f64, f64)]) -> u32 {
    let mut cluster: u32 = 0;

    for (i, c) in centroids.iter().enumerate() {
        let d1: f64 = get_distance(data_point, c);
        let d2: f64 = get_distance(data_point, &centroids[cluster as usize]);

        if d1 < d2 {
            cluster = i as u32;
        }
    }

    cluster
}

pub fn k_means(data_points: Vec<(f64, f64)>, n_clusters: usize, max_iter: i32) -> Option<Vec<u32>> {
    if data_points.len() < n_clusters {
        return None;
    }

    let mut centroids: Vec<(f64, f64)> = Vec::new();
    let mut labels: Vec<u32> = vec![0; data_points.len()];

    for _ in 0..n_clusters {
        let x: f64 = random::<f64>();
        let y: f64 = random::<f64>();

        centroids.push((x, y));
    }

    let mut count_iter: i32 = 0;

    while count_iter < max_iter {
        let mut new_centroids_position: Vec<(f64, f64)> = vec![(0.0, 0.0); n_clusters];
        let mut new_centroids_num: Vec<u32> = vec![0; n_clusters];

        for (i, d) in data_points.iter().enumerate() {
            let nearest_cluster: u32 = find_nearest(d, &centroids);
            labels[i] = nearest_cluster;

            new_centroids_position[nearest_cluster as usize].0 += d.0;
            new_centroids_position[nearest_cluster as usize].1 += d.1;
            new_centroids_num[nearest_cluster as usize] += 1;
        }

        for i in 0..centroids.len() {
            if new_centroids_num[i] == 0 {
                continue;
            }

            let new_x: f64 = new_centroids_position[i].0 / new_centroids_num[i] as f64;
            let new_y: f64 = new_centroids_position[i].1 / new_centroids_num[i] as f64;

            centroids[i] = (new_x, new_y);
        }

        count_iter += 1;
    }

    Some(labels)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_k_means() {
        let mut data_points: Vec<(f64, f64)> = vec![];
        let n_points: usize = 1000;

        for _ in 0..n_points {
            let x: f64 = random::<f64>() * 100.0;
            let y: f64 = random::<f64>() * 100.0;

            data_points.push((x, y));
        }

        println!("{:?}", k_means(data_points, 10, 100).unwrap_or_default());
    }
}
