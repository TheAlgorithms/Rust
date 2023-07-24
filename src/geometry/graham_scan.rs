use crate::geometry::Point;
use std::cmp::Ordering;

fn point_min(a: &&Point, b: &&Point) -> Ordering {
    // Find the bottom-most point. In the case of a tie, find the left-most.
    if a.y == b.y {
        a.x.partial_cmp(&b.x).unwrap()
    } else {
        a.y.partial_cmp(&b.y).unwrap()
    }
}

// Returns a Vec of Points that make up the convex hull of `points`. Returns an empty Vec if there
// is no convex hull.
pub fn graham_scan(mut points: Vec<Point>) -> Vec<Point> {
    if points.len() <= 2 {
        return vec![];
    }

    let min_point = points.iter().min_by(point_min).unwrap().clone();
    points.retain(|p| p != &min_point);
    if points.is_empty() {
        // edge case where all the points are the same
        return vec![];
    }

    let point_cmp = |a: &Point, b: &Point| -> Ordering {
        // Sort points in counter-clockwise direction relative to the min point. We can this by
        // checking the orientation of consecutive vectors (min_point, a) and (a, b).
        let orientation = min_point.consecutive_orientation(a, b);
        if orientation < 0.0 {
            Ordering::Greater
        } else if orientation > 0.0 {
            Ordering::Less
        } else {
            let a_dist = min_point.euclidean_distance(a);
            let b_dist = min_point.euclidean_distance(b);
            // When two points have the same relative angle to the min point, we should only
            // include the further point in the convex hull. We sort further points into a lower
            // index, and in the algorithm, remove all consecutive points with the same relative
            // angle.
            b_dist.partial_cmp(&a_dist).unwrap()
        }
    };
    points.sort_by(point_cmp);
    let mut convex_hull: Vec<Point> = vec![];

    // We always add the min_point, and the first two points in the sorted vec.
    convex_hull.push(min_point.clone());
    convex_hull.push(points[0].clone());
    let mut top = 1;
    for point in points.iter().skip(1) {
        if min_point.consecutive_orientation(point, &convex_hull[top]) == 0.0 {
            // Remove consecutive points with the same angle. We make sure include the furthest
            // point in the convex hull in the sort comparator.
            continue;
        }
        loop {
            // In this loop, we remove points that we determine are no longer part of the convex
            // hull.
            if top <= 1 {
                break;
            }
            // If there is a segment(i+1, i+2) turns right relative to segment(i, i+1), point(i+1)
            // is not part of the convex hull.
            let orientation =
                convex_hull[top - 1].consecutive_orientation(&convex_hull[top], point);
            if orientation <= 0.0 {
                top -= 1;
                convex_hull.pop();
            } else {
                break;
            }
        }
        convex_hull.push(point.clone());
        top += 1;
    }
    if convex_hull.len() <= 2 {
        return vec![];
    }
    convex_hull
}

#[cfg(test)]
mod tests {
    use super::graham_scan;
    use super::Point;

    fn test_graham(convex_hull: Vec<Point>, others: Vec<Point>) {
        let mut points = convex_hull.clone();
        points.append(&mut others.clone());
        let graham = graham_scan(points);
        for point in convex_hull {
            assert!(graham.contains(&point));
        }
        for point in others {
            assert!(!graham.contains(&point));
        }
    }

    #[test]
    fn too_few_points() {
        test_graham(vec![], vec![]);
        test_graham(vec![], vec![Point::new(0.0, 0.0)]);
    }

    #[test]
    fn duplicate_point() {
        let p = Point::new(0.0, 0.0);
        test_graham(vec![], vec![p.clone(), p.clone(), p.clone(), p.clone(), p]);
    }

    #[test]
    fn points_same_line() {
        let p1 = Point::new(1.0, 0.0);
        let p2 = Point::new(2.0, 0.0);
        let p3 = Point::new(3.0, 0.0);
        let p4 = Point::new(4.0, 0.0);
        let p5 = Point::new(5.0, 0.0);
        // let p6 = Point::new(1.0, 1.0);
        test_graham(vec![], vec![p1, p2, p3, p4, p5]);
    }

    #[test]
    fn triangle() {
        let p1 = Point::new(1.0, 1.0);
        let p2 = Point::new(2.0, 1.0);
        let p3 = Point::new(1.5, 2.0);
        let points = vec![p1, p2, p3];
        test_graham(points, vec![]);
    }

    #[test]
    fn rectangle() {
        let p1 = Point::new(1.0, 1.0);
        let p2 = Point::new(2.0, 1.0);
        let p3 = Point::new(2.0, 2.0);
        let p4 = Point::new(1.0, 2.0);
        let points = vec![p1, p2, p3, p4];
        test_graham(points, vec![]);
    }

    #[test]
    fn triangle_with_points_in_middle() {
        let p1 = Point::new(1.0, 1.0);
        let p2 = Point::new(2.0, 1.0);
        let p3 = Point::new(1.5, 2.0);
        let p4 = Point::new(1.5, 1.5);
        let p5 = Point::new(1.2, 1.3);
        let p6 = Point::new(1.8, 1.2);
        let p7 = Point::new(1.5, 1.9);
        let hull = vec![p1, p2, p3];
        let others = vec![p4, p5, p6, p7];
        test_graham(hull, others);
    }

    #[test]
    fn rectangle_with_points_in_middle() {
        let p1 = Point::new(1.0, 1.0);
        let p2 = Point::new(2.0, 1.0);
        let p3 = Point::new(2.0, 2.0);
        let p4 = Point::new(1.0, 2.0);
        let p5 = Point::new(1.5, 1.5);
        let p6 = Point::new(1.2, 1.3);
        let p7 = Point::new(1.8, 1.2);
        let p8 = Point::new(1.9, 1.7);
        let p9 = Point::new(1.4, 1.9);
        let hull = vec![p1, p2, p3, p4];
        let others = vec![p5, p6, p7, p8, p9];
        test_graham(hull, others);
    }

    #[test]
    fn star() {
        // A single stroke star shape (kind of). Only the tips(p1-5) are part of the convex hull. The
        // other points would create angles >180 degrees if they were part of the polygon.
        let p1 = Point::new(-5.0, 6.0);
        let p2 = Point::new(-11.0, 0.0);
        let p3 = Point::new(-9.0, -8.0);
        let p4 = Point::new(4.0, 4.0);
        let p5 = Point::new(6.0, -7.0);
        let p6 = Point::new(-7.0, -2.0);
        let p7 = Point::new(-2.0, -4.0);
        let p8 = Point::new(0.0, 1.0);
        let p9 = Point::new(1.0, 0.0);
        let p10 = Point::new(-6.0, 1.0);
        let hull = vec![p1, p2, p3, p4, p5];
        let others = vec![p6, p7, p8, p9, p10];
        test_graham(hull, others);
    }

    #[test]
    fn rectangle_with_points_on_same_line() {
        let p1 = Point::new(1.0, 1.0);
        let p2 = Point::new(2.0, 1.0);
        let p3 = Point::new(2.0, 2.0);
        let p4 = Point::new(1.0, 2.0);
        let p5 = Point::new(1.5, 1.0);
        let p6 = Point::new(1.0, 1.5);
        let p7 = Point::new(2.0, 1.5);
        let p8 = Point::new(1.5, 2.0);
        let hull = vec![p1, p2, p3, p4];
        let others = vec![p5, p6, p7, p8];
        test_graham(hull, others);
    }
}
