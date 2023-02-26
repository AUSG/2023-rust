// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

use std::f64::consts::PI;
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    // add methods
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn magnitude(&self) -> f64 {
        return f64::sqrt((self.x * self.x + self.y * self.y) as f64);
    }

    fn dist(self, p: Point) -> f64 {
        return f64::sqrt(
            ((self.x - p.x) * (self.x - p.x) + (self.y - p.y) * (self.y - p.y)) as f64,
        );
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub struct Polygon {
    // add fields
    points: Vec<Point>,
}

use std::slice::Iter;
impl Polygon {
    // add methods
    fn new() -> Polygon {
        return Polygon { points: Vec::new() };
    }

    fn add_point(&mut self, p: Point) {
        self.points.push(p);
    }

    fn left_most_point(&self) -> Option<Point> {
        return self
            .points
            .iter()
            .reduce(|a, b| if a.x < b.x { a } else { b })
            .map(|t| t.to_owned());
    }

    fn iter(&self) -> Iter<Point> {
        return self.points.iter();
    }

    fn perimeter(&self) -> f64 {
        match self.points.len() {
            0..=1 => 0 as f64,
            2 => self
                .points
                .first()
                .unwrap()
                .dist(*self.points.last().unwrap()) as f64,
            _ => {
                self.points
                    .windows(2)
                    .map(|pair| pair[0].dist(pair[1]))
                    .reduce(|acc, val| acc + val)
                    .unwrap()
                    + self
                        .points
                        .first()
                        .unwrap()
                        .dist(*self.points.last().unwrap())
            }
        }
    }
}

pub struct Circle {
    // add fields
    center: Point,
    radius: i32,
}

impl Circle {
    // add methods
    fn new(center: Point, radius: i32) -> Circle {
        Circle { center, radius }
    }

    fn perimeter(&self) -> f64 {
        return self.radius as f64 * 2. * PI;
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl From<Polygon> for Shape {
    fn from(poly: Polygon) -> Self {
        Shape::Polygon(poly)
    }
}

impl From<Circle> for Shape {
    fn from(circle: Circle) -> Self {
        Shape::Circle(circle)
    }
}

impl Shape {
    fn perimeter(&self) -> f64 {
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
