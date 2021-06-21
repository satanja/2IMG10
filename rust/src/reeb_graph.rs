use fxhash::FxHashMap;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct CriticalPoint {
    value: usize,
}

impl CriticalPoint {
    pub fn new(value: usize) -> CriticalPoint {
        CriticalPoint { value }
    }
}

#[derive(Debug)]
pub struct ReebGraph {
    slices: FxHashMap<usize, FxHashMap<CriticalPoint, Vec<CriticalPoint>>>,
    x_coords: FxHashMap<CriticalPoint, i32>
}

impl ReebGraph {
    /// Constructs a new Reeb graph given a root node
    pub fn new(root: &CriticalPoint, x: i32, layer: usize) -> ReebGraph {
        let mut root_map = FxHashMap::default();
        let mut edges = FxHashMap::default();
        let mut x_map = FxHashMap::default();
        edges.insert(root.clone(), Vec::new());
        root_map.insert(layer, edges);
        x_map.insert(root.clone(), x);
        ReebGraph { slices: root_map, x_coords: x_map }
    }

    /// Joins a parent to a new critical point `point` and creates a new entry for `point`
    ///
    /// # Panics
    /// Panics when parent has not been inserted into the reeb graph before.
    pub fn add_point(&mut self, layer: usize, parent: &CriticalPoint, point: &CriticalPoint, point_x: i32) {
        match self.slices.get_mut(&layer) {
            Some(_) => {}
            None => {
                let mut edges = FxHashMap::default();
                edges.insert(parent.clone(), Vec::new());
                self.slices.insert(layer, edges);
            }
        }

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

        if let Some(events) = self.slices.get_mut(&layer) {
            match events.get(point) {
                Some(_) => {}
                None => {
                    events.insert(point.clone(), Vec::new());
                }
            }
        } else {
            let mut edge_map = FxHashMap::default();
            edge_map.insert(point.clone(), Vec::new());
            self.slices.insert(layer, edge_map);
        }
    }

    pub fn to_ipe(&mut self) {
        const X_SCALE: i32 = 16;
        const Y_SCALE: usize = 16;
        for (layer, edges) in &self.slices {
            let y = layer * Y_SCALE;
            for (parent, children) in edges {
                for child in children {
                    let child_x = self.x_coords.get(child).unwrap() * X_SCALE;
                    println!("<path layer=\"alpha\" stroke=\"black\">");
                    println!("{} {} m", self.x_coords.get(parent).unwrap() * X_SCALE, y);
                    println!("{} {} l", child_x, y + Y_SCALE);
                    println!("h");
                    println!("</path>");
                    println!("<use name=\"mark/disk(sx)\" pos=\"{} {}\" size=\"normal\" stroke=\"black\"/>", child_x, y + Y_SCALE);
                }
            }
        }
    }
}