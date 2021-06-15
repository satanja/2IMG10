use fxhash::FxHashMap;
use fxhash::FxHashSet;

use crate::geometry::Polygon;
use crate::geometry::DCEL;

#[derive(Clone)]
pub struct NetworkEdge {
    pos: (i32, i32),
    delta: f64,
}

impl NetworkEdge {
    pub fn new(pos: (i32, i32), delta: f64) -> NetworkEdge {
        NetworkEdge { pos, delta }
    }
}

pub struct Graph {
    vertices: FxHashMap<(i32, i32), usize>,
    adj: Vec<Vec<NetworkEdge>>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            vertices: FxHashMap::default(),
            adj: Vec::new(),
        }
    }

    fn get_index(&self, vertex: &(i32, i32)) -> usize {
        *self.vertices.get(vertex).unwrap()
    }

    fn add_vertex(&mut self, vertex: &(i32, i32)) {
        if !self.vertices.contains_key(vertex) {
            self.vertices.insert(vertex.clone(), self.vertices());
            self.adj.push(Vec::new());
        }
    }

    /// Inserts an edge into the graph. Also inserts vertices if they were not present in the graph.
    pub fn add_edge(&mut self, from: (i32, i32), to: (i32, i32), delta: f64) {
        self.add_vertex(&from);
        self.add_vertex(&to);

        let i = self.get_index(&from);
        let j = self.get_index(&to);

        self.adj[i].push(NetworkEdge::new(to, delta));
        self.adj[j].push(NetworkEdge::new(from, delta));
    }

    /// Removes all vertices with one outgoing edge exhaustively
    pub fn reduce(&mut self) {
        loop {
            let mut reduced = false;
            let mut singles: Vec<_> = self
                .vertices
                .iter()
                .filter(|(_, index)| self.adj[**index].len() <= 1)
                .collect();

            if singles.len() > 0 {
                reduced = true;
                singles.sort_by(|(_, a), (_, b)| a.cmp(b));
                let set: FxHashSet<_> = singles.clone().into_iter().map(|(pos, _)| *pos).collect();
                let mut j = 0;

                let mut new_adj = Vec::new();
                for i in 0..self.adj.len() {
                    if j < singles.len() && i == *singles[j].1 {
                        j += 1;
                    } else {
                        let filtered: Vec<_> = self.adj[i]
                            .clone()
                            .into_iter()
                            .filter(|e| !set.contains(&e.pos))
                            .collect();

                        new_adj.push(filtered);
                    }
                }

                let mut good_vertices: Vec<_> = self
                    .vertices
                    .iter()
                    .filter(|(_, index)| self.adj[**index].len() > 1)
                    .collect();
                good_vertices.sort_by(|(_, a), (_, b)| a.cmp(b));

                let mut new_vertices = FxHashMap::default();
                let mut n = 0;
                for (pos, _) in good_vertices {
                    new_vertices.insert(*pos, n);
                    n += 1;
                }

                self.vertices = new_vertices;
                self.adj = new_adj;
            }

            if !reduced {
                break;
            }
        }
    }

    /// Gets the list of edges from `vertex`
    pub fn get_edges(&self, vertex: &(i32, i32)) -> Option<&Vec<NetworkEdge>> {
        if let Some(index) = self.vertices.get(vertex) {
            Some(&self.adj[*index])
        } else {
            None
        }
    }

    /// Returns the number of vertices in the graph
    pub fn vertices(&self) -> usize {
        self.vertices.len()
    }

    /// Returns the number of edges in the graph
    pub fn edges(&self) -> usize {
        self.adj.iter().fold(0, |acc, edges| acc + edges.len()) / 2
    }

    pub fn polygons(&self) -> Vec<Polygon> {
        let mut dcel = DCEL::new();
        for (vertex, index) in &self.vertices {
            for edge in &self.adj[*index] {
                let from = (vertex.0 as f64, vertex.1 as f64);
                let to = (edge.pos.0 as f64, edge.pos.1 as f64);

                dcel.add_line(&from, &to);
            }
        }
        dcel.make_polygons()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_edge() {
        let mut graph = Graph::new();
        graph.add_edge((0, 0), (1, 1), 0.5);
        assert_eq!(graph.vertices(), 2);
        assert_eq!(graph.edges(), 1);
    }

    #[test]
    fn reduce_to_empty() {
        let mut graph = Graph::new();
        graph.add_edge((0, 0), (1, 1), 0.5);
        graph.reduce();
        assert_eq!(graph.vertices(), 0);
        assert_eq!(graph.edges(), 0);
    }

    #[test]
    fn reduce() {
        let mut graph = Graph::new();
        graph.add_edge((0, 0), (1, 1), 0.5);
        graph.add_edge((0, 0), (0, 1), 0.5);
        graph.add_edge((0, 1), (1, 0), 0.5);
        graph.add_edge((0, 0), (1, 0), 0.5);
        graph.reduce();
        assert_eq!(graph.vertices(), 3);
        assert_eq!(graph.edges(), 3);
    }
}
