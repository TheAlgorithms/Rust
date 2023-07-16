/// Shamos-Hoey Algorithm. Given a set of line segments, finds if any two line segments intersect.
/// Some implementations assume that there are no vertical lines. This implementation supports
/// vertical lines.
use super::point::Point;
use super::segment::Segment;
use std::cmp::{Ord, Ordering};
use std::collections::BTreeSet;

enum PointType {
    Start,
    End,
}

type SegmentAndPoint<'a> = (&'a Segment, &'a Point, PointType);
struct SegmentWrapper<'a> {
    segment: &'a Segment,
    // `x` holds the x value when the segment was added to the set. We order segments at point `x`
    // by their y-value at `x`. Note that if the order of two segments ever changes, it means that
    // they intersected, which our algorithm can detect.
    //
    // Ideally we could track a single value of x as it changes through the algorithm iterations.
    // We need to implement `Ord` for the struct so that we could insert it into the RB-tree. The
    // Ord implementation would not have access to a `x` local variable in `shamos_hoey()`. We keep
    // keep track of x in the struct as a workaround. When comparing two segments, we take the max
    // of the two segments' x's, which represents the most up to date `x`.
    x: f64,
    // `point` is used soley for comparisons involving a vertical segment. For most comparisons, we
    // compute the y-value at a given x, but vertical lines would have multiple y-values. For
    // vertical lines, we order them by the endpoint from bottom to top.
    point: &'a Point,
}

impl<'a> SegmentWrapper<'a> {
    fn new(segment: &'a Segment, x: f64, point: &'a Point) -> SegmentWrapper<'a> {
        SegmentWrapper { segment, x, point }
    }
}

impl Segment {
    // Returns (start, end) points
    fn get_start_end(&self) -> (&Point, &Point) {
        match point_cmp(&self.a, &self.b) {
            Ordering::Less | Ordering::Equal => (&self.a, &self.b),
            Ordering::Greater => (&self.b, &self.a),
        }
    }
}

impl<'a> Ord for SegmentWrapper<'a> {
    fn cmp(&self, other: &SegmentWrapper) -> Ordering {
        // For vertical segments, we compare using the endpoint.
        let x = if self.x < other.x { other.x } else { self.x };
        let y1 = if self.segment.is_vertical() {
            self.point.y
        } else {
            self.segment.compute_y_at_x(x)
        };
        let y2 = if other.segment.is_vertical() {
            other.point.y
        } else {
            other.segment.compute_y_at_x(x)
        };
        y1.partial_cmp(&y2).unwrap()
    }
}

impl<'a> PartialOrd for SegmentWrapper<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Point {}
impl<'a> Eq for SegmentWrapper<'a> {}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl<'a> PartialEq for SegmentWrapper<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.segment.a == other.segment.a && self.segment.b == other.segment.b
    }
}

fn point_cmp(p1: &Point, p2: &Point) -> Ordering {
    // We order points from left to right. Points that have the same x value are ordered from
    // bottom to top.
    let cmp = p1.x.partial_cmp(&p2.x).unwrap();
    if cmp == Ordering::Equal {
        p1.y.partial_cmp(&p2.y).unwrap()
    } else {
        cmp
    }
}

fn intersects_with_neighbors(
    segment: &Segment,
    upper: Option<&SegmentWrapper>,
    lower: Option<&SegmentWrapper>,
) -> bool {
    if let Some(upper) = upper {
        if segment.intersects(upper.segment) {
            return true;
        }
    }
    if let Some(lower) = lower {
        if segment.intersects(lower.segment) {
            return true;
        }
    }
    false
}

pub fn shamos_hoey(segments: Vec<&Segment>) -> bool {
    let mut x = f64::MIN;
    let mut points: Vec<SegmentAndPoint> = Vec::new();
    for segment in segments {
        let (start, end) = segment.get_start_end();
        points.push((segment, start, PointType::Start));
        points.push((segment, end, PointType::End));
    }
    points.sort_by(|sp1: &SegmentAndPoint, sp2: &SegmentAndPoint| -> Ordering {
        point_cmp(sp1.1, sp2.1)
    });

    let mut segments: BTreeSet<SegmentWrapper> = BTreeSet::new();

    for (segment, point, pt) in points {
        assert!(
            point.x >= x,
            "When sweeping left to right, the new x must be >= the new x"
        );
        x = point.x;
        let upper = segments
            .range(SegmentWrapper::new(segment, x, point)..)
            .next();
        let lower = segments
            .range(..SegmentWrapper::new(segment, x, point))
            .last();
        match pt {
            PointType::Start => {
                // When adding a segment, check if the new segment intersects with its neighbors.
                if intersects_with_neighbors(segment, upper, lower) {
                    return true;
                }
                segments.insert(SegmentWrapper::new(segment, x, point));
            }
            PointType::End => {
                // When removing a segment, check if it's neightbors intersects.
                if let (Some(upper), Some(lower)) = (upper, lower) {
                    if upper.segment.intersects(lower.segment) {
                        return true;
                    }
                }
                segments.remove(&SegmentWrapper::new(segment, x, point));
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::shamos_hoey;
    use super::Segment;

    #[test]
    fn no_segments() {
        assert!(!shamos_hoey(vec![]));
    }

    #[test]
    fn single_point() {
        assert!(!shamos_hoey(vec![&Segment::new(0.0, 0.0, 0.0, 0.0)]));
    }

    #[test]
    fn one_segment() {
        assert!(!shamos_hoey(vec![&Segment::new(0.0, 0.0, 1.0, 1.0)]));
    }

    #[test]
    fn two_intersecting_segments() {
        let s1 = Segment::new(2.0, 3.0, 6.0, 5.0);
        let s2 = Segment::new(-1.0, 9.0, 10.0, -3.0);
        let vec = vec![&s1, &s2];
        assert!(shamos_hoey(vec));
    }

    #[test]
    fn two_non_intersecting_segments() {
        let s1 = Segment::new(-1.0, 9.0, 10.0, -3.0);
        let s2 = Segment::new(-0.0, 10.0, 11.0, -2.0);
        let vec = vec![&s1, &s2];
        assert!(!shamos_hoey(vec));
    }

    #[test]
    fn intersects_endpoint_on_segment() {
        let s1 = Segment::new(2.0, 3.0, 6.0, 5.0);
        let s2 = Segment::new(4.0, 4.0, -11.0, 20.0);
        let vec = vec![&s1, &s2];
        assert!(shamos_hoey(vec));
    }

    #[test]
    fn intersects_self() {
        let s1 = Segment::new(2.0, 3.0, 6.0, 5.0);
        let s2 = Segment::new(2.0, 3.0, 6.0, 5.0);
        let vec = vec![&s1, &s2];
        assert!(shamos_hoey(vec));
    }

    #[test]
    fn intersects() {
        let s1 = Segment::new(2.0, 3.0, 6.0, 5.0);
        let s2 = Segment::new(-1.0, 9.0, 10.0, -3.0);
        let s3 = Segment::new(100.0, 200.0, 40.0, 50.0);
        let vec = vec![&s1, &s2, &s3];
        assert!(shamos_hoey(vec));
    }

    #[test]
    fn intersects2() {
        let s1 = Segment::new(0.0, 10.0, 10.0, 0.0);
        let s2 = Segment::new(3.0, 9.0, 10.0, 15.0);
        let s3 = Segment::new(6.0, 7.0, 9.0, 0.0);
        let vec = vec![&s1, &s2, &s3];
        assert!(shamos_hoey(vec));
    }

    #[test]
    fn intersects3() {
        // s1 and s2 intersect. the rest have no intersections
        let s1 = Segment::new(1.0, 2.0, 3.0, 4.0);
        let s2 = Segment::new(0.0, 5.0, 2.0, 1.0);
        let s3 = Segment::new(-4.0, 5.0, 0.0, 9.0);
        let s4 = Segment::new(6.0, 7.0, 9.0, 0.0);
        let s5 = Segment::new(10.0, 7.0, 5.0, 15.0);
        let s6 = Segment::new(-10.0, -49.0, -10.0, 2.0);
        let s7 = Segment::new(0.0, 16.0, 6.0, 12.0);
        let s8 = Segment::new(-3.0, 1.0, 7.0, -24.0);
        let vec = vec![&s1, &s2, &s3, &s4, &s5, &s6, &s7, &s8];
        assert!(shamos_hoey(vec));
    }

    #[test]
    fn too_short_to_intersect() {
        let s1 = Segment::new(2.0, 3.0, 6.0, 5.0);
        let s2 = Segment::new(-1.0, 10.0, 3.0, 5.0);
        let s3 = Segment::new(5.0, 3.0, 10.0, -11.0);
        let vec = vec![&s1, &s2, &s3];
        assert!(!shamos_hoey(vec));
    }

    #[test]
    fn single_vertical_line() {
        let s1 = Segment::new(2.0, 3.0, 2.0, -5.0);
        let vec = vec![&s1];
        assert!(!shamos_hoey(vec));
    }

    #[test]
    fn vertical_line_intersects() {
        let s1 = Segment::new(2.0, 3.0, 2.0, -5.0);
        let s2 = Segment::new(-1.0, 10.0, 3.0, -5.0);
        let vec = vec![&s1, &s2];
        assert!(shamos_hoey(vec));

        let s3 = Segment::new(2.0, 1.0, 8.0, -5.0);
        let vec = vec![&s1, &s3];
        assert!(shamos_hoey(vec));
    }

    #[test]
    fn vertical_line_no_intersect() {
        let s1 = Segment::new(2.0, 3.0, 2.0, -5.0);
        let s2 = Segment::new(-1.0, 6.0, 10.0, 20.0);
        let s3 = Segment::new(-4.0, -17.0, 4.0, -4.0);
        let vec = vec![&s1, &s2, &s3];
        assert!(!shamos_hoey(vec));
    }

    #[test]
    fn three_segments_intersect_at_same_point() {
        // All intersect at (1, 3)
        let s1 = Segment::new(-4.0, 3.0, 8.0, 3.0);
        let s2 = Segment::new(-1.0, 0.0, 3.0, 4.5);
        let s3 = Segment::new(-3.0, 5.0, 4.0, 0.5);
        let vec = vec![&s1, &s2, &s3];
        assert!(shamos_hoey(vec));
    }

    #[test]
    fn parallel_segments() {
        let s1 = Segment::new(-5.0, 0.0, 5.0, 0.0);
        let s2 = Segment::new(-5.0, 1.0, 5.0, 1.0);
        let s3 = Segment::new(-5.0, 2.0, 5.0, 2.0);
        let s4 = Segment::new(-5.0, 3.0, 5.0, 3.0);
        let s5 = Segment::new(-5.0, -1.0, 5.0, -1.0);
        let s6 = Segment::new(-5.0, -2.0, 5.0, -2.0);
        let s7 = Segment::new(-5.0, -3.0, 5.0, -3.0);
        let s8 = Segment::new(-5.0, -4.0, 5.0, -4.0);
        let vec = vec![&s1, &s2, &s3, &s4, &s5, &s6, &s7, &s8];
        assert!(!shamos_hoey(vec));
    }
}
