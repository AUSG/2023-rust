use std::ops::{Add};
use std::slice::Iter;
use core::f64::consts::PI;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}
impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn magnitude(&self) -> f64  {
        let square_sum = self.x.pow(2) + self.y.pow(2);
        let result = (square_sum as f64).sqrt();
        result
    }

    pub fn dist(&self, p: Point) -> f64 {
        let unit_vector = Point { x: (self.x - p.x).abs(), y: (self.y - p.y).abs() };
        unit_vector.magnitude()
    }
}

pub struct Polygon {
    points: Vec<Point>
}
impl Polygon {
    pub fn new() -> Polygon {
        Polygon { points: vec![] }
    }

    pub fn add_point(&mut self, p: Point) {
        self.points.push(p);
    }

    pub fn left_most_point(&self) -> Option<Point> {
        Some(self.points[0])
    }

    pub fn iter(&self) -> Iter<Point> {
        self.points.iter()
    }

    pub fn perimeter(&self) -> f64 {
        let mut sum = self.points[0].dist(self.points[self.points.len() - 1]);
        for i in 0..self.points.len()-1 {
            sum += self.points[i].dist(self.points[i+1]);
        }
        sum
    }
}

pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    pub fn new(center: Point, radius: i32) -> Circle {
        Circle { center, radius }
    }

    pub fn perimeter(&self) -> f64 {
        2.0 * PI * (self.radius as f64)
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}
impl From<Polygon> for Shape {
    fn from(shape: Polygon) -> Self {
        Shape::Polygon(shape)
    }
}
impl From<Circle> for Shape {
    fn from(shape: Circle) -> Self {
        Shape::Circle(shape)
    }
}
impl Shape {
    pub fn perimeter(&self) -> f64 {
        match self {
            Shape::Polygon(poly) => poly.perimeter(),
            Shape::Circle(circle) => circle.perimeter(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        print!("{}", round_two_digits(p1.magnitude()));
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_perimeters() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let perimeters = shapes
            .iter()
            .map(Shape::perimeter)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(perimeters, vec![15.48, 31.42]);
    }
}

#[allow(dead_code)]
fn main() {}