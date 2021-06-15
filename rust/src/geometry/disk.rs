#[derive(Debug)]
pub struct Disk {
    center: (f64, f64),
    radius: f64,
    points: Vec<(f64, f64)>,
}

impl Disk {
    pub fn new(center: (f64, f64), radius: f64) -> Disk {
        Disk {
            center,
            radius,
            points: Vec::new(),
        }
    }

    pub fn contains_point(&self, a: &(f64, f64)) -> bool {
        let dxs = (a.0 - self.center.0) * (a.0 - self.center.0);
        let dys = (a.1 - self.center.1) * (a.1 - self.center.1);
        dxs + dys <= self.radius * self.radius
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn centroid(&self) -> (f64, f64) {
        self.center
    }

    // pub fn add_point(&mut self, point: (f64, f64)) {
    //     self.points.push(point);
    // }
}
