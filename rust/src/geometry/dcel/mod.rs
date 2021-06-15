#![allow(dead_code)]

use std::fmt;
mod beachline;
mod point;
mod util;

use crate::geometry::Polygon;
use fxhash::FxHashMap;
use geo::point;
use point::Point;
use util::{segment_intersection, Segment};

const NIL: usize = !0;

/// Doubly Connected Edge List representation of a subdivision of the plane.
pub struct DCEL {
    /// Vertices
    vertices: Vec<Vertex>,
    /// Halfedges
    halfedges: Vec<HalfEdge>,
    /// Faces
    faces: Vec<Face>,

    map: FxHashMap<(i32, i32), usize>,
    adj: Vec<Vec<usize>>,
}

impl DCEL {
    /// Construct an empty DCEL
    pub fn new() -> Self {
        DCEL {
            vertices: vec![],
            halfedges: vec![],
            faces: vec![],
            map: FxHashMap::default(),
            adj: Vec::new(),
        }
    }

    /// Add two halfedges that are twins
    pub fn add_twins(&mut self) -> (usize, usize) {
        let mut he1 = HalfEdge::new();
        let mut he2 = HalfEdge::new();

        let start_index = self.halfedges.len();
        he1.twin = start_index + 1;
        he2.twin = start_index;
        self.halfedges.push(he1);
        self.halfedges.push(he2);
        (start_index, start_index + 1)
    }

    /// Get the origin of a halfedge by index
    pub fn get_origin(&self, edge: usize) -> Point {
        let origin_ind = self.halfedges[edge].origin;
        return self.vertices[origin_ind].coordinates;
    }

    /// Set the previous edge of all halfedges
    /// Assumes that the DCEL is well-formed.
    pub fn set_prev(&mut self) {
        let mut seen_edges = vec![false; self.halfedges.len()];
        for edge_ind in 0..self.halfedges.len() {
            if seen_edges[edge_ind] {
                continue;
            }
            let mut current_ind = edge_ind;
            seen_edges[current_ind];
            loop {
                let next_edge = self.halfedges[current_ind].next;
                self.halfedges[next_edge].prev = current_ind;
                current_ind = next_edge;
                seen_edges[current_ind] = true;
                if current_ind == edge_ind {
                    break;
                }
            }
        }
    }

    fn remove_edge(&mut self, edge: usize) {
        let edge_prev = self.halfedges[edge].prev;
        let edge_next = self.halfedges[edge].next;
        let twin = self.halfedges[edge].twin;
        let twin_prev = self.halfedges[twin].prev;
        let twin_next = self.halfedges[twin].next;

        self.halfedges[edge_prev].next = twin_next;
        self.halfedges[edge_next].prev = twin_prev;
        self.halfedges[twin_prev].next = edge_next;
        self.halfedges[twin_next].prev = edge_prev;

        self.halfedges[edge].alive = false;
        self.halfedges[twin].alive = false;
    }

    fn get_edges_around_vertex(&self, vertex: usize) -> Vec<usize> {
        let mut result = vec![];
        let start_edge = self.vertices[vertex].incident_edge;
        let mut current_edge = start_edge;
        loop {
            result.push(current_edge);
            let current_twin = self.halfedges[current_edge].twin;
            current_edge = self.halfedges[current_twin].next;
            if current_edge == start_edge {
                break;
            }
        }
        return result;
    }

    /// Remove a vertex and all attached halfedges.
    /// Does not affect faces!!
    pub fn remove_vertex(&mut self, vertex: usize) {
        let vertex_edges = self.get_edges_around_vertex(vertex);
        for edge in vertex_edges {
            self.remove_edge(edge);
        }
        self.vertices[vertex].alive = false;
    }

    pub fn add_edge_unchecked(&mut self, a: &(i32, i32), b: &(i32, i32)) {

        let twins = self.add_twins();
        if !self.map.contains_key(a) {
            self.map.insert(*a, self.vertices.len());
            let v = Vertex {
                coordinates: Point::new(a.0 as f64, a.1 as f64),
                incident_edge: twins.0,
                alive: true,
            };
            self.vertices.push(v);
            self.adj.push(Vec::new());
        }

        if !self.map.contains_key(b) {
            self.map.insert(*b, self.vertices.len());
            let v = Vertex {
                coordinates: Point::new(b.0 as f64, b.1 as f64),
                incident_edge: twins.1,
                alive: true,
            };
            self.vertices.push(v);
            self.adj.push(Vec::new());
        }
        
        let i = *self.map.get(a).unwrap();
        let j = *self.map.get(b).unwrap();

        self.adj[i].push(twins.1);
        self.adj[j].push(twins.0);
        
        self.halfedges[twins.0].origin = i;
        self.halfedges[twins.1].origin = j;
    }

    pub fn build(&mut self) {
        for i in 0..self.vertices.len() {
            let c = &self.vertices[i];

            let mut sorted = self.adj[i].clone();
            sorted.sort_by(|u, v| {
                let e1 = &self.halfedges[*u];
                let e2 = &self.halfedges[*v];

                let p = &self.vertices[e1.origin];
                let q = &self.vertices[e2.origin];

                let a = p.coordinates - c.coordinates;
                let b = q.coordinates - c.coordinates;

                let theta1 = if a.x() < 0. {
                    (a.y() / a.x()).atan() + std::f64::consts::PI
                } else {
                    (a.y() / a.x()).atan()
                };

                let theta2 = if b.x() < 0. {
                    (b.y() / b.x()).atan() + std::f64::consts::PI
                } else {
                    (b.y() / b.x()).atan()
                };

                theta1.partial_cmp(&theta2).unwrap()
            });

            for j in 0..sorted.len() - 1 {
                let a = sorted[j];
                let b = sorted[j + 1];
                let e1 = self.halfedges[a].twin;

                self.halfedges[e1].next = b;
                self.halfedges[b].prev = e1;
            }

            let last = sorted[sorted.len() - 1];
            let e1 = self.halfedges[last].twin;
            self.halfedges[e1].next = sorted[0];
            self.halfedges[sorted[0]].prev = e1;
        }

        for e in &self.halfedges {
            if e.next == NIL {
                println!("{}", e.origin);
            }
        }
    }

    // does not handle the case where line goes through dcel vertex
    /// Add a line segment to a DCEL.
    ///
    /// Vertices and halfedges are constructed and mutated as necessary.
    /// Faces are not affected. This should be used before add_faces.
    pub fn add_line(&mut self, a: &(f64, f64), b: &(f64, f64)) {
        let p = Point::new(a.0, a.1);
        let q = Point::new(b.0, b.1);
        let seg = [p, q];

        let mut intersections = get_line_intersections(seg, self);
        intersections.sort_by(|a, b| a.0.cmp(&b.0));
        let start_pt = if seg[0] < seg[1] { seg[0] } else { seg[1] };
        let end_pt = if seg[0] < seg[1] { seg[1] } else { seg[0] };

        let (mut line_needs_next, mut line_needs_prev, _) = add_twins_from_pt(start_pt, self);
        self.halfedges[line_needs_prev].next = line_needs_next;
        let prev_pt = start_pt;

        for (int_pt, this_cut_edge) in intersections {
            let (new_line_needs_next, new_line_needs_prev, new_pt_ind) =
                add_twins_from_pt(int_pt, self);
            self.halfedges[line_needs_prev].origin = new_pt_ind;

            let mut cut_edge = this_cut_edge;
            if makes_left_turn(prev_pt, int_pt, self.get_origin(this_cut_edge)) {
                cut_edge = self.halfedges[cut_edge].twin;
            }

            let old_cut_next = self.halfedges[cut_edge].next;
            let old_cut_twin = self.halfedges[cut_edge].twin;
            self.halfedges[cut_edge].next = line_needs_prev;

            let cut_ext_ind = self.halfedges.len();
            let cut_ext_he = HalfEdge {
                origin: new_pt_ind,
                next: old_cut_next,
                twin: old_cut_twin,
                face: NIL,
                prev: NIL,
                alive: true,
            };
            self.halfedges.push(cut_ext_he);
            self.halfedges[line_needs_next].next = cut_ext_ind;

            let old_twin_next = self.halfedges[old_cut_twin].next;
            self.halfedges[old_cut_twin].next = new_line_needs_next;

            let twin_ext_ind = self.halfedges.len();
            let twin_ext_he = HalfEdge {
                origin: new_pt_ind,
                next: old_twin_next,
                twin: cut_edge,
                face: NIL,
                prev: NIL,
                alive: true,
            };
            self.halfedges.push(twin_ext_he);
            self.halfedges[new_line_needs_prev].next = twin_ext_ind;

            self.halfedges[cut_edge].twin = twin_ext_ind;
            self.halfedges[old_cut_twin].twin = cut_ext_ind;

            line_needs_next = new_line_needs_next;
            line_needs_prev = new_line_needs_prev;
        }

        self.halfedges[line_needs_next].next = line_needs_prev;
        let end_vertex_ind = self.vertices.len();
        let end_vertex = Vertex {
            coordinates: end_pt,
            incident_edge: line_needs_prev,
            alive: true,
        };
        self.vertices.push(end_vertex);
        self.halfedges[line_needs_prev].origin = end_vertex_ind;
    }

    /// Construct faces for a DCEL.
    ///
    /// # Panics
    ///
    /// This method will panic if the DCEL has any faces already.
    pub fn add_faces(&mut self) {
        if !self.faces.is_empty() {
            panic!("add_faces only works on DCELs with no faces");
        }
        let num_halfedges = self.halfedges.len();
        let mut seen_edges = vec![false; num_halfedges];

        // info!("Adding faces. There are {} halfedges.", num_halfedges);

        for edge_index in 0..num_halfedges {
            if seen_edges[edge_index] || !self.halfedges[edge_index].alive {
                continue;
            }

            let face_index = self.faces.len();
            let new_face = Face::new(edge_index);
            self.faces.push(new_face);

            let mut current_edge = edge_index;
            loop {
                seen_edges[current_edge] = true;
                self.halfedges[current_edge].face = face_index;
                current_edge = self.halfedges[current_edge].next;
                if current_edge == edge_index {
                    break;
                }
            }
        }
        // info!("Generated faces for {} edges.", processed_edges);
    }

    /// Constructs the polygons from the faces
    pub fn make_polygons(&self) -> Vec<Polygon> {
        let mut result = vec![];
        for face in &self.faces {
            if !face.alive {
                continue;
            }
            let mut this_poly = vec![];
            let start_edge = face.outer_component;
            let mut current_edge = start_edge;
            loop {
                this_poly.push(self.get_origin(current_edge));
                current_edge = self.halfedges[current_edge].next;
                if current_edge == start_edge {
                    break;
                }
            }
            result.push(this_poly);
        }

        // remove the outer face
        result.sort_by(|a, b| a.len().cmp(&b.len()));
        result.pop();

        result
            .into_iter()
            .map(|list| {
                let points = list.into_iter().map(|p| (p.x(), p.y())).collect();
                Polygon::new(points)
            })
            .collect()
    }
}

impl fmt::Debug for DCEL {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut vertices_disp = String::new();

        for (index, node) in self.vertices.iter().enumerate() {
            if node.alive {
                vertices_disp.push_str(format!("{}: {:?}\n", index, node).as_str());
            }
        }

        let mut faces_disp = String::new();

        for (index, node) in self.faces.iter().enumerate() {
            if node.alive {
                faces_disp.push_str(format!("{}: {}\n", index, node).as_str());
            }
        }

        let mut halfedges_disp = String::new();

        for (index, node) in self.halfedges.iter().enumerate() {
            if node.alive {
                halfedges_disp.push_str(format!("{}: {:?}\n", index, node).as_str());
            }
        }

        write!(
            f,
            "Vertices:\n{}\nFaces:\n{}\nHalfedges:\n{}",
            vertices_disp, faces_disp, halfedges_disp
        )
    }
}

/// A vertex of a DCEL
pub struct Vertex {
    /// (x, y) coordinates
    coordinates: Point,
    /// Some halfedge having this vertex as the origin
    incident_edge: usize, // index of halfedge
    /// False if the vertex has been deleted
    alive: bool,
}

impl fmt::Debug for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}, edge: {}", self.coordinates, self.incident_edge)
    }
}

/// A halfedge of a DCEL
pub struct HalfEdge {
    /// The index of the vertex at the start of the halfedge
    pub origin: usize, // index of vertex
    /// The index of the twin halfedge
    pub twin: usize, // index of halfedge
    /// The index of the next halfedge
    pub next: usize, // index of halfedge
    face: usize, // index of face
    prev: usize, // index of halfedge
    alive: bool,
}

impl fmt::Debug for HalfEdge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "origin: {}, twin: {}, next: {}",
            self.origin, self.twin, self.next
        )
    }
}

impl HalfEdge {
    /// Construct an empty halfedge
    pub fn new() -> Self {
        HalfEdge {
            origin: NIL,
            twin: NIL,
            next: NIL,
            face: NIL,
            prev: NIL,
            alive: true,
        }
    }
}

#[derive(Debug)]
/// A face of a DCEL
pub struct Face {
    outer_component: usize, // index of halfedge
    alive: bool,
}

impl fmt::Display for Face {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "outer: {}", self.outer_component)
    }
}

impl Face {
    /// Construct a new face, given an attached halfedge index
    pub fn new(edge: usize) -> Self {
        Face {
            outer_component: edge,
            alive: true,
        }
    }
}

/// Do the three points, in this order, make a left turn?
pub fn makes_left_turn(pt1: Point, pt2: Point, pt3: Point) -> bool {
    let x1 = pt1.x();
    let x2 = pt2.x();
    let x3 = pt3.x();
    let y1 = pt1.y();
    let y2 = pt2.y();
    let y3 = pt3.y();

    (x2 - x1) * (y3 - y1) - (y2 - y1) * (x3 - x1) > 0.
}

fn add_twins_from_pt(start_pt: Point, dcel: &mut DCEL) -> (usize, usize, usize) {
    let (twin1, twin2) = dcel.add_twins();

    let start_vertex = Vertex {
        coordinates: start_pt,
        incident_edge: twin1,
        alive: true,
    };
    let start_vertex_ind = dcel.vertices.len();
    dcel.vertices.push(start_vertex);

    dcel.halfedges[twin1].origin = start_vertex_ind;

    (twin1, twin2, start_vertex_ind)
}

fn get_line_intersections(seg: Segment, dcel: &DCEL) -> Vec<(Point, usize)> {
    let mut intersections = vec![];
    let mut seen_halfedges = vec![false; dcel.halfedges.len()];
    for (index, halfedge) in dcel.halfedges.iter().enumerate() {
        let twin = halfedge.twin;
        if seen_halfedges[index] || seen_halfedges[twin] || !halfedge.alive {
            continue;
        }
        let this_seg = [dcel.get_origin(index), dcel.get_origin(twin)];
        let this_intersection = segment_intersection(seg, this_seg);
        if let Some(int_pt) = this_intersection {
            intersections.push((int_pt, index));
        }
        seen_halfedges[index] = true;
        seen_halfedges[twin] = true;
    }
    return intersections;
}

/// Constructs the line segments of the Voronoi diagram.
pub fn make_line_segments(dcel: &DCEL) -> Vec<Segment> {
    let mut result = vec![];
    for halfedge in &dcel.halfedges {
        if halfedge.origin != NIL && halfedge.next != NIL && halfedge.alive {
            if dcel.halfedges[halfedge.next].origin != NIL {
                result.push([
                    dcel.vertices[halfedge.origin].coordinates,
                    dcel.get_origin(halfedge.next),
                ])
            }
        }
    }
    result
}
