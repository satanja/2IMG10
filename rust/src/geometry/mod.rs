mod disk;
mod line;
use line::Line;
pub use disk::Disk;

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

fn trivial_triple(a: &(f64, f64), b: &(f64, f64), c: &(f64, f64)) -> Option<Disk> {
    let center_ab = ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0);
    let (left_ab, right_ab) = if a.0 <= b.0 { (a, b) } else { (b, a) };
    let perp_slope_ab = -1.0 * (right_ab.0 - left_ab.0) / (right_ab.1 - left_ab.1);
    let l1 = Line::new(perp_slope_ab, center_ab);

    let center_cb = ((c.0 + b.0) / 2.0, (c.1 + b.1) / 2.0);
    let (left_cb, right_cb) = if c.0 <= b.0 { (c, b) } else { (b, c) };
    let perp_slope_cb = -1.0 * (right_cb.0 - left_cb.0) / (right_cb.1 - left_cb.1);
    let l2 = Line::new(perp_slope_cb, center_cb);

    if l1.is_overlapping_with(&l2) || l1.is_parallel_to(&l2) {
        return None;
    }

    if let Some(center) = l1.intersection(&l2) {
        let radius = dist(&a, &center);

        let d = Disk::new(center, radius);
        Some(d)
    } else {
        return None;
    }
}

pub fn smallest_disk(
    enclosing: &mut Vec<(f64, f64)>,
    boundary: &mut Vec<(f64, f64)>,
) -> Option<Disk> {
    if enclosing.len() == 0 || boundary.len() == 3 {
        match boundary.len() {
            1 => return Some(trivial_point(&boundary[0])),
            2 => return Some(trivial_pair(&boundary[0], &boundary[1])),
            3 => return trivial_triple(&boundary[0], &boundary[1], &boundary[2]),
            _ => return None,
        }
    }

    let p = enclosing.pop().unwrap();
    if let Some(d) = smallest_disk(&mut enclosing.clone(), &mut boundary.clone()) {
        if d.contains_point(&p) {
            return Some(d);
        }
    }

    boundary.push(p);
    return smallest_disk(enclosing, boundary);
}
