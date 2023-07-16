pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    pub fn cross_prod(&self, other: &Point) -> f64 {
        self.x * other.y - self.y * other.x
    }
}
