use std::iter::FromIterator;

use geo::algorithm::contains::Contains;
use geo::concave_hull::ConcaveHull;
use geo::point;
use geo::prelude::Centroid;
use geo::LineString;
use geo::Point;
use geo::simplify::Simplify;
use geo_svg::ToSvg;

use crate::geometry::smallest_disk;
use geo::algorithm::concave_hull;

pub struct Polygon {
    vertices: Vec<(f64, f64)>,
    polygon: geo::Polygon<f64>,
}

impl Polygon {
    /// Constructs a new Polygon
    pub fn new(vertices: Vec<(f64, f64)>) -> Polygon {
        let line_string =
            LineString::from_iter(vertices.clone().into_iter().map(|(x, y)| Point::new(x, y)));
        let mut polygon = geo::Polygon::new(line_string, vec![]);
        // polygon = polygon.concave_hull(1.0);
        Polygon { vertices, polygon }
    }

    /// Returns wether `self` contains `point`
    pub fn contains(&self, point: &(f64, f64)) -> bool {
        self.polygon.contains(&point!(x: point.0, y: point.1))
    }

    /// Returns the centroid of all the vertices
    pub fn centroid(&self) -> Option<(f64, f64)> {
        if let Some(c) = self.polygon.centroid() {
            return Some((c.x(), c.y()));
        }
        None
    }

    /// Returns the smallest enclosing disk of the vertices
    pub fn smallest_disk_centroid(&self) -> Option<(f64, f64)> {
        let mut enclosing = self.vertices.clone();
        let mut boundary = Vec::new();
        match smallest_disk(&mut enclosing, &mut boundary) {
            Some(disk) => Some(disk.centroid()),
            None => None,
        }
    }

    pub fn to_svg(&self) {
        let svg = self.polygon.to_svg().with_radius(0.4).with_color(geo_svg::Color::Named("green"));
        println!("{}", svg);
    }

    pub fn to_ipe(&self) {
        println!("<path layer=\"alpha\" stroke=\"black\">");
        let simplified: Vec<_> = self.polygon.exterior().points_iter().collect();
        for i in 0..simplified.len() {
            let x = simplified[i].x();
            let y = simplified[i].y();
            if i == 0 {
                println!("{} {} m", x, y);
            } else {
                println!("{} {} l", x, y);
            }
        }
        println!("h");
        println!("</path>");
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
