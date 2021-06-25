use fxhash::FxHashMap;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct CriticalPoint {
    value: i32,
}

impl CriticalPoint {
    pub fn new(value: i32) -> CriticalPoint {
        CriticalPoint { value }
    }
}

#[derive(Debug)]
pub struct ReebGraph {
    slices: FxHashMap<i32, FxHashMap<CriticalPoint, Vec<CriticalPoint>>>,
    x_coords: FxHashMap<CriticalPoint, f64>,
    degrees: FxHashMap<CriticalPoint, i32>,
    root: CriticalPoint
}

impl ReebGraph {
    /// Constructs a new Reeb graph given a root node
    pub fn new(root: &CriticalPoint, x: f64, layer: i32) -> ReebGraph {
        let mut root_map = FxHashMap::default();
        let mut edges = FxHashMap::default();
        let mut x_map = FxHashMap::default();
        let mut deg = FxHashMap::default();
        edges.insert(root.clone(), Vec::new());
        root_map.insert(layer, edges);
        x_map.insert(root.clone(), x);
        deg.insert(root.clone(), 1);
        ReebGraph { slices: root_map, x_coords: x_map, degrees: deg, root: root.clone() }
    }

    /// Joins a parent to a new critical point `point` and creates a new entry for `point`
    ///
    /// # Panics
    /// Panics when parent has not been inserted into the reeb graph before.
    pub fn add_point(&mut self, layer: i32, parent: &CriticalPoint, point: &CriticalPoint, point_x: f64) {
        // layer
        match self.slices.get_mut(&layer) {
            Some(_) => {}
            None => {
                let mut edges = FxHashMap::default();
                edges.insert(parent.clone(), Vec::new());
                self.slices.insert(layer, edges);
            }
        }

        // parent in layer
        match self.slices.get_mut(&layer).unwrap().get_mut(&parent) {
            Some(p) => {
                p.push(point.clone());
            }
            None => {
                let layer = self.slices.get_mut(&layer).unwrap();
                layer.insert(parent.clone(), Vec::new());
                layer.get_mut(&parent).unwrap().push(point.clone());
            }
        }

        match self.x_coords.get_mut(&point.clone()) {
            Some(_) => {}
            None => {
                self.x_coords.insert(point.clone(), point_x);
            }
        }

        match self.degrees.get_mut(&point.clone()) {
            Some(d) => {
                *d += 1;
            }
            None => {
                self.degrees.insert(point.clone(), 1);
            }
        }

        match self.degrees.get_mut(&parent.clone()) {
            Some(d) => {
                *d += 1;
            }
            None => {
                self.degrees.insert(parent.clone(), 1);
            }
        }
    }

    pub fn to_ipe(&mut self) {
        const X_SCALE: f64 = 16.;
        const Y_SCALE: i32 = 16;
        for (layer, edges) in &self.slices {
            let y = layer * Y_SCALE;
            for (parent, children) in edges {
                let parent_x = self.x_coords.get(parent).unwrap() * X_SCALE;
                for child in children {
                    let child_x = self.x_coords.get(child).unwrap() * X_SCALE;
                    println!("<path layer=\"alpha\" stroke=\"black\">");
                    println!("{} {} m", parent_x, y);
                    println!("{} {} l", child_x, y + Y_SCALE);
                    println!("h");
                    println!("</path>");
                    if *self.degrees.get(child).unwrap() != 2 {
                        println!("<use name=\"mark/disk(sx)\" pos=\"{} {}\" size=\"normal\" stroke=\"black\"/>", child_x, y + Y_SCALE);
                    }
                }
                if parent.value == self.root.value {
                    println!("<use name=\"mark/disk(sx)\" pos=\"{} {}\" size=\"normal\" stroke=\"black\"/>", parent_x, y);
                }
            }
        }
    }
}