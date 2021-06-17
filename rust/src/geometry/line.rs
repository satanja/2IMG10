pub struct Line {
    slope: Option<f64>,
    contact: (f64, f64),
    intercept: Option<f64>,
}

use geo::Point;

impl Line {
    pub fn from_points(a: &Point<f64>, b: &Point<f64>) -> Line {
        let left = if a.x() <= b.x() { a } else { b };
        let right = if a.x() <= b.x() { b } else { a };

        let slope = (right.y() - left.y()) / (right.x() - left.x());
        Line::new(slope, (a.x(), a.y()))
    }

    pub fn new(slope: f64, contact: (f64, f64)) -> Line {
        let s = if slope.is_infinite() {
            None
        } else {
            Some(slope)
        };

        let intercept = if let Some(a) = s {
            Some(contact.1 - a * contact.0)
        } else {
            None
        };

        Line {
            slope: s,
            contact,
            intercept,
        }
    }

    pub fn evaluate(&self, x: f64) -> Option<f64> {
        if let Some(a) = self.slope {
            Some(a * x + self.intercept.unwrap())
        } else {
            if x == self.contact.0 {
                Some(x)
            } else {
                None
            }
        }
    }

    pub fn intersection(&self, other: &Self) -> Option<(f64, f64)> {
        if let Some(a1) = self.slope {
            if let Some(a2) = other.slope {
                let b1 = self.intercept.unwrap();
                let b2 = other.intercept.unwrap();

                let x = (b2 - b1) / (a1 - a2);
                let y = self.evaluate(x).unwrap();

                Some((x, y))
            } else {
                other.intersection(&self)
            }
        } else {
            let x = self.contact.0;
            if let Some(y) = other.evaluate(x) {
                Some((x, y))
            } else {
                None
            }
        }
    }

    pub fn is_overlapping_with(&self, other: &Self) -> bool {
        if let Some(a1) = self.slope {
            if let Some(a2) = other.slope {
                a1 == a2 && self.intercept.unwrap() == other.intercept.unwrap()
            } else {
                false
            }
        } else {
            if let Some(_) = other.slope {
                other.is_overlapping_with(&self)
            } else {
                self.contact.0 == other.contact.0
            }
        }
    }

    pub fn is_parallel_to(&self, other: &Self) -> bool {
        if let Some(a1) = self.slope {
            if let Some(a2) = other.slope {
                a1 == a2
            } else {
                false
            }
        } else {
            if let Some(_) = other.slope {
                other.is_parallel_to(&self)
            } else {
                true
            }
        }
    }
}
