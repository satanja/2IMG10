use fxhash::FxHashMap;
use fxhash::FxHashSet;

use crate::geometry::Polygon;
use crate::geometry::DCEL;

#[derive(Clone)]
pub struct NetworkEdge {
    index: usize,
    delta: f64,
}

impl NetworkEdge {
    pub fn new(index: usize, delta: f64) -> NetworkEdge {
        NetworkEdge { index, delta }
    }
}

pub struct Graph {
    set_vertices: FxHashMap<(i32, i32), usize>,
    vertices: Vec<(i32, i32)>,
    adj: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            set_vertices: FxHashMap::default(),
            vertices: Vec::new(),
            adj: Vec::new(),
        }
    }

    fn get_index(&self, vertex: &(i32, i32)) -> usize {
        *self.set_vertices.get(vertex).unwrap()
    }

    pub fn add_vertex(&mut self, vertex: (i32, i32)) {
        self.vertices.push(vertex);
        self.adj.push(Vec::new());
    }

    /// Inserts an edge into the graph. Also inserts vertices if they were not present in the graph.
    pub fn add_edge(&mut self, from: usize, to: usize, delta: f64) {
        if let Err(index) = self.adj[from].binary_search(&to) {
            self.adj[from].insert(index, to);
        }
        if to == self.adj.len() {
            println!("shit");
        }

        if let Err(index) = self.adj[to].binary_search(&from) {
            self.adj[to].insert(index, from);
        }
    }

    #[allow(dead_code)]
    /// Removes all vertices with one outgoing edge exhaustively
    pub fn reduce(&mut self) {
        // TODO fix indices
        loop {
            let mut reduced = false;
            let singles: Vec<_> = (0..self.vertices.len())
                .filter(|index| self.adj[*index].len() <= 1)
                .map(|index| self.vertices[index])
                .collect();

            if singles.len() > 0 {
                reduced = true;

                let good_vertices: Vec<_> = (0..self.vertices.len())
                    .filter(|index| self.adj[*index].len() > 1)
                    .collect();

                self.vertices = good_vertices
                    .clone()
                    .into_iter()
                    .map(|index| self.vertices[index])
                    .collect();

                let mut set_vertices = FxHashMap::default();
                for vertex in &good_vertices {
                    set_vertices.insert(*vertex, set_vertices.len());
                }

                self.adj = good_vertices
                    .iter()
                    .map(|index| {
                        self.adj[*index]
                            .iter()
                            .filter(|j| set_vertices.contains_key(*j))
                            .map(|j| *set_vertices.get(j).unwrap())
                            .collect()
                    })
                    .collect();
            }

            if !reduced {
                break;
            }
        }
    }

    /// Gets the list of edges from `vertex`
    pub fn get_edges(&self, vertex: &(i32, i32)) -> Option<&Vec<usize>> {
        if let Some(index) = self.set_vertices.get(vertex) {
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

    pub fn is_empty(&self) -> bool {
        self.vertices() == 0 && self.edges() == 0
    }

    pub fn polygons(&self) -> Vec<Polygon> {
        let mut dcel = DCEL::new();

        for i in 0..self.adj.len() {
            for j in 0..self.adj[i].len() {
                if self.adj[i][j] <= i {
                    continue;
                }
                let from = &self.vertices[i];
                let k = self.adj[i][j];

                let to = &self.vertices[k];
                dcel.add_edge_unchecked(from, to);
            }
        }

        dcel.build();
        dcel.add_faces();
        dcel.make_polygons()
    }

    pub fn to_ipe(&self) {
        println!("<ipeselection pos=\"0 0\">");
        // for vertex in &self.vertices {
        //     println!("<use layer=\"alpha\" name=\"mark/disk(sx)\" pos=\"{} {}\" size=\"normal\" stroke=\"black\"/>",vertex.0, vertex.1);
        // }

        for i in 0..self.adj.len() {
            for j in 0..self.adj[i].len() {
                if self.adj[i][j] <= i {
                    continue;
                }
                let from = &self.vertices[i];
                let k = self.adj[i][j];

                let to = &self.vertices[k];
                println!("<path layer=\"alpha\" stroke=\"black\">");
                println!("{} {} m", from.0, from.1);
                println!("{} {} l", to.0, to.1);
                println!("</path>");
            }
        }
        println!("</ipeselection>");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_edge() {
        let mut graph = Graph::new();
        graph.add_vertex((0, 0));
        graph.add_vertex((1, 1));
        graph.add_edge(0, 1, 0.5);
        assert_eq!(graph.vertices(), 2);
        assert_eq!(graph.edges(), 1);
    }

    #[test]
    fn reduce_to_empty() {
        let mut graph = Graph::new();
        graph.add_vertex((0, 0));
        graph.add_vertex((1, 1));
        graph.add_edge(0, 1, 0.5);
        graph.reduce();
        assert_eq!(graph.vertices(), 0);
        assert_eq!(graph.edges(), 0);
    }

    #[test]
    fn reduce() {
        let mut graph = Graph::new();
        graph.add_vertex((0, 0));
        graph.add_vertex((0, 1));
        graph.add_vertex((1, 0));
        graph.add_vertex((1, 1));
        graph.add_edge(0, 3, 0.5);
        graph.add_edge(0, 1, 0.5);
        graph.add_edge(1, 2, 0.5);
        graph.add_edge(0, 2, 0.5);
        graph.reduce();
        assert_eq!(graph.vertices(), 3);
        assert_eq!(graph.edges(), 3);
    }

    #[test]
    fn polygons() {
        let mut graph = Graph::new();
        graph.add_vertex((0, 0));
        graph.add_vertex((0, 1));
        graph.add_vertex((1, 0));
        graph.add_vertex((1, 1));
        graph.add_vertex((2, 0));
        graph.add_vertex((2, 1));
        graph.add_edge(0, 1, 1.);
        graph.add_edge(0, 2, 1.);
        graph.add_edge(1, 2, 1.);
        graph.add_edge(1, 4, 1.);
        graph.add_edge(2, 3, 1.);
        graph.add_edge(2, 5, 1.);
        graph.add_edge(4, 5, 1.);

        assert_eq!(graph.polygons().len(), 2);
    }
}
