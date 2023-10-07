use crate::geometry::Point;
use std::cmp::Ordering;

fn cmp_x(p1: &Point, p2: &Point) -> Ordering {
    let acmp = f64_cmp(&p1.x, &p2.x);
    match acmp {
        Ordering::Equal => f64_cmp(&p1.y, &p2.y),
        _ => acmp,
    }
}

fn f64_cmp(a: &f64, b: &f64) -> Ordering {
    a.partial_cmp(b).unwrap()
}

/// returns the two closest points
/// or None if there are zero or one point
pub fn closest_points(points: &[Point]) -> Option<(Point, Point)> {
    let mut points_x: Vec<Point> = points.to_vec();
    points_x.sort_by(cmp_x);
    let mut points_y = points_x.clone();
    points_y.sort_by(|p1: &Point, p2: &Point| -> Ordering { p1.y.partial_cmp(&p2.y).unwrap() });

    closest_points_aux(&points_x, points_y, 0, points_x.len())
}

// We maintain two vectors with the same points, one sort by x coordinates and one sorted by y
// coordinates.
fn closest_points_aux(
    points_x: &[Point],
    points_y: Vec<Point>,
    mut start: usize,
    mut end: usize,
) -> Option<(Point, Point)> {
    let n = end - start;

    if n <= 1 {
        return None;
    }

    if n <= 3 {
        // bruteforce
        let mut min = points_x[0].euclidean_distance(&points_x[1]);
        let mut pair = (points_x[0].clone(), points_x[1].clone());

        for i in 1..n {
            for j in (i + 1)..n {
                let new = points_x[i].euclidean_distance(&points_x[j]);
                if new < min {
                    min = new;
                    pair = (points_x[i].clone(), points_x[j].clone());
                }
            }
        }
        return Some(pair);
    }

    let mid = start + (end - start) / 2;
    let mid_x = points_x[mid].x;

    // Separate points into y_left and y_right vectors based on their x-coordinate. Since y is
    // already sorted by y_axis, y_left and y_right will also be sorted.
    let mut y_left = vec![];
    let mut y_right = vec![];
    for point in &points_y {
        if point.x < mid_x {
            y_left.push(point.clone());
        } else {
            y_right.push(point.clone());
        }
    }

    let left = closest_points_aux(points_x, y_left, start, mid);
    let right = closest_points_aux(points_x, y_right, mid, end);

    let (mut min_sqr_dist, mut pair) = match (left, right) {
        (Some((l1, l2)), Some((r1, r2))) => {
            let dl = l1.euclidean_distance(&l2);
            let dr = r1.euclidean_distance(&r2);
            if dl < dr {
                (dl, (l1, l2))
            } else {
                (dr, (r1, r2))
            }
        }
        (Some((a, b)), None) => (a.euclidean_distance(&b), (a, b)),
        (None, Some((a, b))) => (a.euclidean_distance(&b), (a, b)),
        (None, None) => unreachable!(),
    };

    let dist = min_sqr_dist;
    while points_x[start].x < mid_x - dist {
        start += 1;
    }
    while points_x[end - 1].x > mid_x + dist {
        end -= 1;
    }

    for (i, e) in points_y.iter().enumerate() {
        for k in 1..8 {
            if i + k >= points_y.len() {
                break;
            }

            let new = e.euclidean_distance(&points_y[i + k]);
            if new < min_sqr_dist {
                min_sqr_dist = new;
                pair = ((*e).clone(), points_y[i + k].clone());
            }
        }
    }

    Some(pair)
}

#[cfg(test)]
mod tests {
    use super::closest_points;
    use super::Point;

    fn eq(p1: Option<(Point, Point)>, p2: Option<(Point, Point)>) -> bool {
        match (p1, p2) {
            (None, None) => true,
            (Some((p1, p2)), Some((p3, p4))) => (p1 == p3 && p2 == p4) || (p1 == p4 && p2 == p3),
            _ => false,
        }
    }

    macro_rules! assert_display {
        ($left: expr, $right: expr) => {
            assert!(
                eq($left, $right),
                "assertion failed: `(left == right)`\nleft: `{:?}`,\nright: `{:?}`",
                $left,
                $right
            )
        };
    }

    #[test]
    fn zero_points() {
        let vals: [Point; 0] = [];
        assert_display!(closest_points(&vals), None::<(Point, Point)>);
    }

    #[test]
    fn one_points() {
        let vals = [Point::new(0., 0.)];
        assert_display!(closest_points(&vals), None::<(Point, Point)>);
    }

    #[test]
    fn two_points() {
        let vals = [Point::new(0., 0.), Point::new(1., 1.)];
        assert_display!(
            closest_points(&vals),
            Some((vals[0].clone(), vals[1].clone()))
        );
    }

    #[test]
    fn three_points() {
        let vals = [Point::new(0., 0.), Point::new(1., 1.), Point::new(3., 3.)];
        assert_display!(
            closest_points(&vals),
            Some((vals[0].clone(), vals[1].clone()))
        );
    }

    #[test]
    fn list_1() {
        let vals = [
            Point::new(0., 0.),
            Point::new(2., 1.),
            Point::new(5., 2.),
            Point::new(2., 3.),
            Point::new(4., 0.),
            Point::new(0., 4.),
            Point::new(5., 6.),
            Point::new(4., 4.),
            Point::new(7., 3.),
            Point::new(-1., 2.),
            Point::new(2., 6.),
        ];
        assert_display!(
            closest_points(&vals),
            Some((Point::new(2., 1.), Point::new(2., 3.)))
        );
    }

    #[test]
    fn list_2() {
        let vals = [
            Point::new(1., 3.),
            Point::new(4., 6.),
            Point::new(8., 8.),
            Point::new(7., 5.),
            Point::new(5., 3.),
            Point::new(10., 3.),
            Point::new(7., 1.),
            Point::new(8., 3.),
            Point::new(4., 9.),
            Point::new(4., 12.),
            Point::new(4., 15.),
            Point::new(7., 14.),
            Point::new(8., 12.),
            Point::new(6., 10.),
            Point::new(4., 14.),
            Point::new(2., 7.),
            Point::new(3., 8.),
            Point::new(5., 8.),
            Point::new(6., 7.),
            Point::new(8., 10.),
            Point::new(6., 12.),
        ];
        assert_display!(
            closest_points(&vals),
            Some((Point::new(4., 14.), Point::new(4., 15.)))
        );
    }

    #[test]
    fn vertical_points() {
        let vals = [
            Point::new(0., 0.),
            Point::new(0., 50.),
            Point::new(0., -25.),
            Point::new(0., 40.),
            Point::new(0., 42.),
            Point::new(0., 100.),
            Point::new(0., 17.),
            Point::new(0., 29.),
            Point::new(0., -50.),
            Point::new(0., 37.),
            Point::new(0., 34.),
            Point::new(0., 8.),
            Point::new(0., 3.),
            Point::new(0., 46.),
        ];
        assert_display!(
            closest_points(&vals),
            Some((Point::new(0., 40.), Point::new(0., 42.)))
        );
    }
}
