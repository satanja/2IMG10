use std::iter::FromIterator;

use geo::algorithm::contains::Contains;
use geo::point;
use geo::prelude::Centroid;
use geo::LineString;
use geo::Point;

use crate::geometry::smallest_disk;

pub struct Polygon {
    vertices: Vec<(f64, f64)>,
    polygon: geo::Polygon<f64>,
}

impl Polygon {
    pub fn new(vertices: Vec<(f64, f64)>) -> Polygon {
        let line_string =
            LineString::from_iter(vertices.clone().into_iter().map(|(x, y)| Point::new(x, y)));
        let polygon = geo::Polygon::new(line_string, vec![]);
        Polygon { vertices, polygon }
    }

    pub fn contains(&self, point: &(f64, f64)) -> bool {
        self.polygon.contains(&point!(x: point.0, y: point.1))
    }

    pub fn centroid(&self) -> Option<(f64, f64)> {
        if let Some(c) = self.polygon.centroid() {
            return Some((c.x(), c.y()));
        }
        None
    }

    pub fn smallest_disk_centroid(&self) -> Option<(f64, f64)> {
        let mut enclosing = self.vertices.clone();
        let mut boundary = Vec::new();
        match smallest_disk(&mut enclosing, &mut boundary) {
            Some(disk) => Some(disk.centroid()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains() {
        let polygon = Polygon::new(vec![(0., 0.), (1., 0.), (1., 1.), (0., 1.)]);
        assert_eq!(polygon.contains(&(0.5, 0.5)), true);
        assert_eq!(polygon.contains(&(-0.5, 0.5)), false);
    }

    #[test]
    fn centroid() {
        let polygon = Polygon::new(vec![(0., 0.), (1., 0.), (1., 1.), (0., 1.)]);
        assert_eq!(polygon.centroid(), Some((0.5, 0.5)));
    }

    #[test]
    fn smallest_disk_centroid() {
        let polygon = Polygon::new(vec![
            (0., 0.),
            (1., 0.),
            (1., 0.5),
            (0.25, 0.25),
            (0.5, 1.),
            (0., 1.),
        ]);
        assert_eq!(polygon.smallest_disk_centroid(), Some((0.5, 0.5)));
    }
}
