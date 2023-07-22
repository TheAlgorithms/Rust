use std::ops::Sub;

#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    // Returns the orientation of consecutive segments ab and bc.
    pub fn consecutive_orientation(&self, b: &Point, c: &Point) -> f64 {
        let p1 = b - self;
        let p2 = c - self;
        p1.cross_prod(&p2)
    }

    pub fn cross_prod(&self, other: &Point) -> f64 {
        self.x * other.y - self.y * other.x
    }

    pub fn euclidean_distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, other: Self) -> Point {
        let x = self.x - other.x;
        let y = self.y - other.y;
        Point::new(x, y)
    }
}
