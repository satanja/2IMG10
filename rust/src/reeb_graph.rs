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
}

impl ReebGraph {
    /// Constructs a new Reeb graph given a root node
    pub fn new(root: &CriticalPoint) -> ReebGraph {
        let mut root_map = FxHashMap::default();
        let mut edges = FxHashMap::default();
        edges.insert(root.clone(), Vec::new());
        root_map.insert(root.value, edges);
        ReebGraph { slices: root_map }
    }

    /// Joins a parent to a new critical point `point` and creates a new entry for `point`
    ///
    /// # Panics
    /// Panics when parent has not been inserted into the reeb graph before.
    pub fn add_point(&mut self, parent: &CriticalPoint, point: &CriticalPoint) {
        let edges = self
            .slices
            .get_mut(&parent.value)
            .unwrap()
            .get_mut(&parent)
            .unwrap();
        edges.push(point.clone());

        if let Some(events) = self.slices.get_mut(&point.value) {
            match events.get(point) {
                Some(_) => {}
                None => {
                    events.insert(point.clone(), Vec::new());
                }
            }
        } else {
            let mut edge_map = FxHashMap::default();
            edge_map.insert(point.clone(), Vec::new());
            self.slices.insert(point.value, edge_map);
        }
    }
}
