use rand::distributions::{Distribution, Standard};
use std::ops::Deref;

#[derive(Clone, Copy, Debug)]
pub struct Point(nalgebra::Point2<f64>);

impl Deref for Point {
    type Target = nalgebra::Point2<f64>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Point {
    #[inline]
    pub fn new(x: f64, y: f64) -> Self {
        Point(nalgebra::Point2::new(x, y))
    }

    #[inline]
    pub fn is_ccw(p0: &Point, p1: &Point, p2: &Point) -> bool {
        (p1.y - p0.y) * (p2.x - p1.x) - (p1.x - p0.x) * (p2.y - p1.y) < 0.0
    }

    #[inline]
    pub fn nearly_equals(p1: &Point, p2: &Point) -> bool {
        (p1.x - p2.x).abs() <= std::f64::EPSILON && (p1.y - p2.y).abs() <= std::f64::EPSILON
    }

    #[inline]
    fn circumdelta(p0: &Point, p1: &Point, p2: &Point) -> (f64, f64) {
        let ax = p0.x;
        let ay = p0.y;
        let bx = p1.x;
        let by = p1.y;
        let cx = p2.x;
        let cy = p2.y;

        let dx = bx - ax;
        let dy = by - ay;
        let ex = cx - ax;
        let ey = cy - ay;

        let bl = dx * dx + dy * dy;
        let cl = ex * ex + ey * ey;
        let d = 0.5 / (dx * ey - dy * ex);

        ((ey * bl - dy * cl) * d, (dx * cl - ex * bl) * d)
    }

    #[inline]
    pub fn circumradius(p0: &Point, p1: &Point, p2: &Point) -> f64 {
        let (x, y) = Point::circumdelta(p0, p1, p2);
        x * x + y * y
    }

    #[inline]
    pub fn circumcenter(p0: &Point, p1: &Point, p2: &Point) -> Point {
        let (x, y) = Point::circumdelta(p0, p1, p2);
        Point::new(p0.x + x, p0.y + y)
    }

    #[inline]
    pub fn in_circle(a: &Point, b: &Point, c: &Point, p: &Point) -> bool {
        let dx = a.x - p.x;
        let dy = a.y - p.y;
        let ex = b.x - p.x;
        let ey = b.y - p.y;
        let fx = c.x - p.x;
        let fy = c.y - p.y;

        let ap = dx * dx + dy * dy;
        let bp = ex * ex + ey * ey;
        let cp = fx * fx + fy * fy;

        dx * (ey * cp - bp * fy) - dy * (ex * cp - bp * fx) + ap * (ex * fy - ey * fx) < 0.0
    }
}

impl Distribution<Point> for Standard
where
    Standard: Distribution<f64>,
{
    #[inline]
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Point {
        Point::new(rng.gen(), rng.gen())
    }
}
