use crate::geometry::Point;

pub fn ramer_douglas_peucker(polygon: &[Point], epsilon: f64) -> Vec<Point> {
    let mut dmax = 0.0;
    let mut index = 0;
    let end = polygon.len() - 1;

    for i in 1..end {
        let d =
            perpendicular_distance(polygon[i].clone(), polygon[0].clone(), polygon[end].clone());
        if d > dmax {
            index = i;
            dmax = d;
        }
    }

    if dmax > epsilon {
        let mut results = ramer_douglas_peucker(&Vec::from(&polygon[..=index]), epsilon);
        results.pop();
        results.extend(ramer_douglas_peucker(
            &Vec::from(&polygon[index..]),
            epsilon,
        ));
        results
    } else {
        vec![polygon[0].clone(), polygon[end].clone()]
    }
}

fn perpendicular_distance(p: Point, a: Point, b: Point) -> f64 {
    let num = (b.y - a.y) * p.x - (b.x - a.x) * p.y + b.x * a.y - b.y * a.x;
    let den = a.euclidean_distance(&b);
    num.abs() / den
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perpendicular_distance() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(0.0, 3.0);
        let p = Point::new(4.0, 0.0);
        assert_eq!(perpendicular_distance(p, a, b), 4.0);
    }

    #[test]
    fn test_ramer_douglas_peucker() {
        // This test doesn't make sense, once you change the epsilon value, the result will change. It is hard to predict the result.
        let a = Point::new(0.0, 0.0);
        let b = Point::new(1.0, 0.0);
        let c = Point::new(2.0, 0.0);
        let d = Point::new(2.0, 1.0);
        let e = Point::new(2.0, 2.0);
        let f = Point::new(1.0, 2.0);
        let g = Point::new(0.0, 2.0);
        let h = Point::new(0.0, 1.0);
        let polygon = vec![
            a.clone(),
            b,
            c.clone(),
            d,
            e.clone(),
            f,
            g.clone(),
            h.clone(),
        ];
        let epsilon = 0.7;
        let result = ramer_douglas_peucker(&polygon, epsilon);
        assert_eq!(result, vec![a, c, e, g, h]);
    }
}
