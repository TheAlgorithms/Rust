use crate::geometry::Point;
use crate::geometry::Segment;

// Returns a Vec of Points that make up the convex hull of `points`. Returns an empty Vec if there
// is no convex hull.
pub fn jarvis_march(points: Vec<Point>) -> Vec<Point> {
    if points.len() <= 2 {
        return vec![];
    }

    let mut convex_hull = vec![];
    let mut left_point = 0;
    for i in 1..points.len() {
        // Find the initial point, which is the leftmost point. In the case of a tie, we take the
        // bottom-most point. This helps prevent adding colinear points on the last segment to the hull.
        if points[i].x < points[left_point].x
            || (points[i].x == points[left_point].x && points[i].y < points[left_point].y)
        {
            left_point = i;
        }
    }
    convex_hull.push(points[left_point].clone());

    let mut p = left_point;
    loop {
        // Find the next counter-clockwise point.
        let mut next_p = (p + 1) % points.len();
        for i in 0..points.len() {
            let orientation = points[p].consecutive_orientation(&points[i], &points[next_p]);
            if orientation > 0.0 {
                next_p = i;
            }
        }

        if next_p == left_point {
            // Completed constructing the hull. Exit the loop.
            break;
        }
        p = next_p;

        let last = convex_hull.len() - 1;
        if convex_hull.len() > 1
            && Segment::from_points(points[p].clone(), convex_hull[last - 1].clone())
                .on_segment(&convex_hull[last])
        {
            // If the last point lies on the segment with the new point and the second to last
            // point, we can remove the last point from the hull.
            convex_hull[last] = points[p].clone();
        } else {
            convex_hull.push(points[p].clone());
        }
    }

    if convex_hull.len() <= 2 {
        return vec![];
    }
    let last = convex_hull.len() - 1;
    if Segment::from_points(convex_hull[0].clone(), convex_hull[last - 1].clone())
        .on_segment(&convex_hull[last])
    {
        // Check for the edge case where the last point lies on the segment with the zero'th and
        // second the last point. In this case, we remove the last point from the hull.
        convex_hull.pop();
        if convex_hull.len() == 2 {
            return vec![];
        }
    }
    convex_hull
}

#[cfg(test)]
mod tests {
    use super::jarvis_march;
    use super::Point;

    fn test_jarvis(convex_hull: Vec<Point>, others: Vec<Point>) {
        let mut points = others.clone();
        points.append(&mut convex_hull.clone());
        let jarvis = jarvis_march(points);
        for point in convex_hull {
            assert!(jarvis.contains(&point));
        }
        for point in others {
            assert!(!jarvis.contains(&point));
        }
    }

    #[test]
    fn too_few_points() {
        test_jarvis(vec![], vec![]);
        test_jarvis(vec![], vec![Point::new(0.0, 0.0)]);
    }

    #[test]
    fn duplicate_point() {
        let p = Point::new(0.0, 0.0);
        test_jarvis(vec![], vec![p.clone(), p.clone(), p.clone(), p.clone(), p]);
    }

    #[test]
    fn points_same_line() {
        let p1 = Point::new(1.0, 0.0);
        let p2 = Point::new(2.0, 0.0);
        let p3 = Point::new(3.0, 0.0);
        let p4 = Point::new(4.0, 0.0);
        let p5 = Point::new(5.0, 0.0);
        // let p6 = Point::new(1.0, 1.0);
        test_jarvis(vec![], vec![p1, p2, p3, p4, p5]);
    }

    #[test]
    fn triangle() {
        let p1 = Point::new(1.0, 1.0);
        let p2 = Point::new(2.0, 1.0);
        let p3 = Point::new(1.5, 2.0);
        let points = vec![p1, p2, p3];
        test_jarvis(points, vec![]);
    }

    #[test]
    fn rectangle() {
        let p1 = Point::new(1.0, 1.0);
        let p2 = Point::new(2.0, 1.0);
        let p3 = Point::new(2.0, 2.0);
        let p4 = Point::new(1.0, 2.0);
        let points = vec![p1, p2, p3, p4];
        test_jarvis(points, vec![]);
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
        test_jarvis(hull, others);
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
        test_jarvis(hull, others);
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
        test_jarvis(hull, others);
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
        test_jarvis(hull, others);
    }
}
