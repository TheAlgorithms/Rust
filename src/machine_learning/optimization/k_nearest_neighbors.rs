#![allow(dead_code)]
#![allow(unused_variables)]

use crate::math::{abs, square_root};
use num_traits::pow;
use std::{
    cmp,
    collections::{BTreeMap, HashMap},
};
#[derive(Debug, Clone)]
pub struct Point {
    data: Vec<f64>,
    label: String,
}

impl Point {
    pub fn build(data: Vec<f64>, label: String) -> Self {
        Self { data, label }
    }
}

fn eucledian_distance(source_point: &Point, destination_point: &Point) -> f64 {
    let mut distance = 0.0;

    if destination_point.data.len() == source_point.data.len() {
        for i in 0..destination_point.data.len() {
            distance += pow(
                destination_point.data.get(i).unwrap() - source_point.data.get(i).unwrap(),
                2,
            );
        }
    }
    distance = square_root(distance);
    abs(distance)
}

pub fn classify_using_knn(
    neighbors: Vec<&Point>, // The training data which essentially consists of a set of points on the X-Y axis represented as vector of Points,
    input_point: Point,     // The input point requiring classification
    k: usize, // The value of 'K'. For example, if K equals 4, classification is determined by the majority vote among the 4 nearest neighbors
    distance_computation: fn(source_point: &Point, destination_point: &Point) -> f64, // An enum employed to specify the technique/formula for calculating the distance between two points
) -> Point {
    // fetch k nearest neighbors for the input_point
    let k_nearest_neighbors =
        fetch_k_nearest_neighbors(&neighbors, &input_point, &k, distance_computation);

    //find the majority amongst these k nearest neigbors
    let label = find_majority_label(k_nearest_neighbors).to_owned();

    //update the input point with the right label after classification
    Point::build(input_point.data, label)
}

// This method identifies the label that appears most frequently among the k nearest neighbors
fn find_majority_label(k_nearest_neighbors: Vec<&Point>) -> String {
    let mut majority_counter_map: HashMap<String, i64> = HashMap::new();
    let mut max_counter = 0;
    let mut label: String = String::new();
    for neighbor in k_nearest_neighbors {
        let mut counter: i64 = 1;
        if majority_counter_map.contains_key(&neighbor.label) {
            counter = majority_counter_map.get(&neighbor.label).unwrap() + 1;
        }
        if counter > max_counter {
            max_counter = counter;
            neighbor.label.clone_into(&mut label);
        }
        majority_counter_map.insert(neighbor.label.to_owned(), counter);
    }
    label
}

//Input is a 1.list of Points in the 2 dimensional array
//2. target point for which classification needs to be done
//3. using a slice here instead of an array. We need to know the size of an array at compile time which is not possible and hence a slice is used here which borrow
fn fetch_k_nearest_neighbors<'a>(
    neighbors: &'a Vec<&'a Point>,
    input_point: &'a Point,
    k: &usize,
    distance_computation: fn(source_point: &Point, destination_point: &Point) -> f64,
) -> Vec<&'a Point> {
    //calculate the distance to all neighbors from input_point.
    let distance_map: BTreeMap<i64, &Point> =
        calculate_distance_to_neighbors(neighbors, input_point, distance_computation);

    //find the k nearest neighbors
    let k_nearest_neighbors: Vec<&Point> = return_k_closest_neighbors(distance_map, k);

    k_nearest_neighbors
}

fn calculate_distance_to_neighbors<'a>(
    neighbors: &'a Vec<&'a Point>,
    input_point: &'a Point,
    distance_computation: fn(source_point: &Point, destination_point: &Point) -> f64,
) -> BTreeMap<i64, &'a Point> {
    let mut distance_map: BTreeMap<i64, &Point> = BTreeMap::new();

    for neighbor in neighbors {
        let distance: f64 = distance_computation(input_point, neighbor);
        //multiplying by 100 since floating point numbers cant be keys. Any precission more than 2 digits is ignored
        distance_map.insert((distance * 100.0) as i64, neighbor);
    }
    distance_map
}

fn return_k_closest_neighbors<'a>(
    mut sorted_distance_map: BTreeMap<i64, &'a Point>,
    k: &usize,
) -> Vec<&'a Point> {
    let mut k_closest_neighbor_vectors: Vec<&Point> = Vec::new();
    for _i in 0..cmp::min(sorted_distance_map.len(), *k) {
        let entry = sorted_distance_map.pop_first().unwrap();
        k_closest_neighbor_vectors.push(entry.1);
    }
    k_closest_neighbor_vectors
}

#[cfg(test)]
mod tests {
    use super::*;

    fn construct_point_data(data: Vec<f64>, label: String) -> Point {
        Point { data, label }
    }

    #[test]
    fn test_fetch_k_nearest_neighbors() {
        let point_1 = construct_point_data(vec![2.0, 2.2], String::from("B"));

        let point_2 = construct_point_data(vec![3.0, 2.2], String::from("A"));

        let point_3 = construct_point_data(vec![1.0, 2.2], String::from("A"));

        let point_4 = construct_point_data(vec![0.1, 2.2], String::from("B"));

        let point_5 = construct_point_data(vec![0.2, 2.2], String::from("B"));

        let neighbors: Vec<&Point> = vec![&point_1, &point_2, &point_3, &point_4, &point_5];
        let input_point = construct_point_data(vec![1.0, 2.2], String::new());
        let k: usize = 2;
        let result = fetch_k_nearest_neighbors(&neighbors, &input_point, &k, eucledian_distance);
        assert_eq!(result.len(), 2);
        let test_point_1 = &point_5;
        assert_eq!(result.get(1).unwrap().data, test_point_1.data);
    }

    #[test]
    fn test_fetch_k_nearest_neighbors_for_3_dimension_data() {
        let point_1 = construct_point_data(vec![1.0, 2.2, 2.0], String::from("B"));

        let point_2 = construct_point_data(vec![3.0, 2.2, 1.0], String::from("A"));

        let point_3 = construct_point_data(vec![1.0, 2.2, 1.0], String::from("A"));

        let point_4 = construct_point_data(vec![0.1, 2.2, 1.0], String::from("B"));

        let point_5 = construct_point_data(vec![0.2, 2.2, 1.0], String::from("B"));

        let neighbors: Vec<&Point> = vec![&point_1, &point_2, &point_3, &point_4, &point_5];
        let input_point = construct_point_data(vec![1.0, 2.2, 3.0], String::new());
        let k: usize = 2;
        let result = fetch_k_nearest_neighbors(&neighbors, &input_point, &k, eucledian_distance);
        assert_eq!(result.len(), 2);
        let test_point_1 = &point_1;
        assert_eq!(result.first().unwrap().data, test_point_1.data);
        let test_point_2 = &point_3;
        assert_eq!(result.get(1).unwrap().data, test_point_2.data);
    }

    #[test]
    fn test_fetch_k_nearest_neighbors_for_negative_signed_input() {
        let point_1 = construct_point_data(vec![2.0, 2.2], String::from("B"));

        let point_2 = construct_point_data(vec![3.0, 2.2], String::from("A"));

        let point_3 = construct_point_data(vec![1.0, 2.2], String::from("A"));

        let point_4 = construct_point_data(vec![0.1, 2.2], String::from("B"));

        let point_5 = construct_point_data(vec![0.2, 2.2], String::from("B"));

        let neighbors: Vec<&Point> = vec![&point_1, &point_2, &point_3, &point_4, &point_5];
        let input_point = construct_point_data(vec![-1.0, -2.2], String::new());
        let k: usize = 2;
        let result = fetch_k_nearest_neighbors(&neighbors, &input_point, &k, eucledian_distance);
        assert_eq!(result.len(), 2);

        let first_closest_point = &point_4;
        assert_eq!(result.first().unwrap().data, first_closest_point.data);

        let second_closest_point = &point_5;
        assert_eq!(result.get(1).unwrap().data, second_closest_point.data);
    }

    #[test]
    fn test_fetch_k_nearest_neighbors_for_duplicate_inputs() {
        let point_1 = construct_point_data(vec![0.0, 2.2], String::from("B"));

        let point_2 = construct_point_data(vec![0.0, 2.2], String::from("A"));

        let point_3 = construct_point_data(vec![0.0, 2.2], String::from("A"));

        let point_4 = construct_point_data(vec![0.0, 2.2], String::from("B"));

        let point_5 = construct_point_data(vec![0.0, 2.2], String::from("B"));

        let neighbors: Vec<&Point> = vec![&point_1, &point_2, &point_3, &point_4, &point_5];
        let input_point = construct_point_data(vec![-1.0, -2.2], String::new());
        let k: usize = 2;

        let result = fetch_k_nearest_neighbors(&neighbors, &input_point, &k, eucledian_distance);
        assert_eq!(result.len(), 1);

        let first_closest_point = &point_4;
        assert_eq!(result.first().unwrap().data, first_closest_point.data);
    }

    #[test]
    fn test_classify_input_for_2_nearest_neighbors() {
        let point_1 = construct_point_data(vec![1.0, 2.2], String::from("B"));

        let point_2 = construct_point_data(vec![1.0, 2.2], String::from("A"));

        let point_3 = construct_point_data(vec![2.0, 2.2], String::from("A"));

        let point_4 = construct_point_data(vec![3.0, 2.2], String::from("A"));

        let point_5 = construct_point_data(vec![4.0, 2.2], String::from("A"));

        let neighbors: Vec<&Point> = vec![&point_1, &point_2, &point_3, &point_4, &point_5];

        let input_point = construct_point_data(vec![-1.0, -2.2], String::new());

        let classified_input_point =
            classify_using_knn(neighbors, input_point.clone(), 2, eucledian_distance);

        assert_eq!(classified_input_point.clone().label, point_2.label);
    }

    #[test]
    fn test_classify_input_for_4_nearest_neighbors() {
        let point_1 = construct_point_data(vec![0.0, 2.2], String::from("B"));

        let point_2 = construct_point_data(vec![1.0, 2.2], String::from("A"));

        let point_3 = construct_point_data(vec![2.0, 2.2], String::from("B"));

        let point_4 = construct_point_data(vec![3.0, 2.2], String::from("B"));

        let point_5 = construct_point_data(vec![4.0, 2.2], String::from("A"));

        let neighbors: Vec<&Point> = vec![&point_1, &point_2, &point_3, &point_4, &point_5];

        let input_point = construct_point_data(vec![-1.0, -2.2], String::new());

        let classified_input_point =
            classify_using_knn(neighbors, input_point.clone(), 4, eucledian_distance);

        assert_eq!(classified_input_point.clone().label, String::from("B"));
    }

    #[test]
    fn test_classify_input_for_equal_majority() {
        let point_1 = construct_point_data(vec![0.0, 2.2], String::from("B"));

        let point_2 = construct_point_data(vec![1.0, 2.2], String::from("A"));

        let point_3 = construct_point_data(vec![2.0, 2.2], String::from("A"));

        let point_4 = construct_point_data(vec![3.0, 2.2], String::from("B"));

        let point_5 = construct_point_data(vec![4.0, 2.2], String::from("C"));

        let neighbors: Vec<&Point> = vec![&point_1, &point_2, &point_3, &point_4, &point_5];

        let input_point = construct_point_data(vec![-1.0, -2.2], String::new());

        let classified_input_point =
            classify_using_knn(neighbors, input_point.clone(), 4, eucledian_distance);

        assert_ne!(classified_input_point.clone().label, String::from("C"));
    }
}
