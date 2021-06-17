use std::iter::FromIterator;

use geo::algorithm::contains::Contains;
use geo::concave_hull::ConcaveHull;
use geo::point;
use geo::prelude::Centroid;
use geo::simplify::Simplify;
use geo::LineString;
use geo::Point;
use geo_svg::ToSvg;

use crate::geometry::smallest_disk;
use geo::algorithm::concave_hull;

pub struct Polygon {
    vertices: Vec<(f64, f64)>,
    polygon: geo::Polygon<f64>,
}

use crate::geometry::Line;

impl Polygon {
    /// Constructs a new Polygon
    pub fn new(vertices: Vec<(f64, f64)>) -> Polygon {
        let simplified = Polygon::simplify(vertices);
        let line_string = LineString::from_iter(
            simplified
                .clone()
                .into_iter()
                .map(|(x, y)| Point::new(x, y)),
        );
        let polygon = geo::Polygon::new(line_string, vec![]);
        Polygon {
            vertices: simplified,
            polygon,
        }
    }

    /// Removes colinear points across the boundary of the polygon
    fn simplify(vertices: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
        let mut simplified = Vec::new();
        for i in 0..vertices.len() {
            if simplified.len() < 2 {
                simplified.push(vertices[i]);
            } else {
                let a = Point::from(simplified[simplified.len() - 2]);
                let mid = Point::from(simplified[simplified.len() - 1]);
                let b = Point::from(vertices[i]);

                let l1 = Line::from_points(&a, &mid);
                let l2 = Line::from_points(&mid, &b);

                if l1.is_overlapping_with(&l2) {
                    // points are colinear
                    simplified.pop();
                }

                simplified.push(vertices[i]);
            }
        }

        simplified
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
        let svg = self
            .polygon
            .to_svg()
            .with_radius(0.4)
            .with_color(geo_svg::Color::Named("green"));
        println!("{}", svg);
    }

    pub fn to_ipe(&self) {
        println!("<path layer=\"alpha\" stroke=\"black\">");
        let boundary: Vec<_> = self.polygon.exterior().points_iter().collect();
        for i in 0..boundary.len() {
            let x = boundary[i].x();
            let y = boundary[i].y();
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
