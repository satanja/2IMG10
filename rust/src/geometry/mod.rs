mod dcel;
mod disk;
mod line;
mod polygon;

pub use dcel::DCEL;
pub use disk::Disk;
use line::Line;
pub use polygon::Polygon;

fn dist(a: &(f64, f64), b: &(f64, f64)) -> f64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    let dxs = dx * dx;
    let dys = dy * dy;
    (dxs + dys).sqrt()
}

fn trivial_point(a: &(f64, f64)) -> Disk {
    Disk::new(a.clone(), 0.0)
}

fn trivial_pair(a: &(f64, f64), b: &(f64, f64)) -> Disk {
    let center = ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0);
    let radius = dist(a, b) / 2.0;
    Disk::new(center, radius)
}

fn trivial_triple(a: &(f64, f64), b: &(f64, f64), c: &(f64, f64)) -> Disk {
    let px = b.0 - a.0;
    let py = b.1 - a.1;
    let qx = c.0 - a.0;
    let qy = c.1 - a.1;

    let u = px * px + py * py;
    let v = qx * qx + qy * qy;
    let w = px * qy - py * qx;

    let z = ((qy * u - py * v) / (2. * w), ((px * v - qx * u) / (2. * w)));
    let center = (z.0 + a.0, z.1 + a.1);
    Disk::new(center, dist(&center, &a))
}

fn is_valid(disk: &Disk, boundary: &Vec<(f64, f64)>) -> bool {
    for point in boundary {
        if !disk.contains_point(point) {
            return false;
        }
    }
    return true;
}

fn trivial_disk(boundary: &Vec<(f64, f64)>) -> Option<Disk> {
    if boundary.len() == 0 {
        return None;
    }

    if boundary.len() == 1 {
        return Some(trivial_point(&boundary[0]));
    }

    if boundary.len() == 2 {
        return Some(trivial_pair(&boundary[0], &boundary[1]));
    }

    for i in 0..boundary.len() {
        for j in i + 1..boundary.len() {
            let a = &boundary[i];
            let b = &boundary[j];
            let disk = trivial_pair(a, b);
            if is_valid(&disk, boundary) {
                return Some(disk);
            }
        }
    }

    Some(trivial_triple(&boundary[0], &boundary[1], &boundary[2]))
}

/// Computes the smallest enclosing disk of points `enclosing` (`boundary` should initially be empty), returns `None` if it does not exist
pub fn smallest_disk(
    enclosing: &Vec<(f64, f64)>,
    boundary: &mut Vec<(f64, f64)>,
    len: usize,
) -> Option<Disk> {
    if len == 0 || boundary.len() == 3 {
        return trivial_disk(boundary)
    }

    let p = enclosing[len - 1];
    if let Some(d) = smallest_disk(&enclosing, &mut boundary.clone(), len - 1) {
        if d.contains_point(&p) {
            return Some(d);
        }
    }

    boundary.push(p);
    return smallest_disk(&enclosing, boundary, len - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trivial_triple_test() {
        let a = (0., 0.);
        let b = (1., 0.);
        let c = (0., 1.);

        assert_eq!(trivial_triple(&a, &b, &c).centroid(), (0.5, 0.5));
    }
}
