use std::fs::{read, File};
use std::io::Result;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use crate::graph::Graph;
use fxhash::{FxHashMap, FxHashSet};

pub fn read_network(delta: f64, path: &PathBuf) -> Result<Graph> {
    let file = File::open(path)?;

    let mut graph = Graph::new();
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    // number of vertices
    let n = line.trim().parse::<usize>().unwrap();
    line.clear();

    // the actual vertices
    let mut vertices = FxHashMap::default();

    for _ in 0..n {
        reader.read_line(&mut line).unwrap();
        let coords: Vec<_> = line.trim().split(' ').collect();
        let x = coords[1].parse::<i32>().unwrap();
        let y = coords[2].parse::<i32>().unwrap();

        
        if !vertices.contains_key(&(x, y)) {
            vertices.insert((x, y), vertices.len());
            graph.add_vertex((x, y));
        }
        line.clear();
    }

    reader.read_line(&mut line).unwrap();
    // the number of edges
    let m = line.trim().parse::<usize>().unwrap();
    line.clear();

    for _ in 0..m {
        reader.read_line(&mut line).unwrap();
        // let data: Vec<_> = line.trim().split(' ').collect(); slow af
        let mut data_iter = line.trim().split(' ');
        data_iter.next();
        data_iter.next();
        data_iter.next();
        let w = data_iter.next().unwrap();
        let path: Vec<_> = data_iter.map(|s| s.parse::<i32>().unwrap()).collect();
        let weight = if w == "nan" {
            f64::NAN
        } else {
            w.parse::<f64>().unwrap()
        };
        if !weight.is_nan() && weight >= delta {
            for v in (0..path.len() - 3).step_by(2) {
                let x1 = path[v];
                let y1 = path[v + 1];

                let x2 = path[v + 2];
                let y2 = path[v + 3];

                let from = match vertices.get(&(x1, y1)) {
                    Some(i) => *i,
                    None => {
                        vertices.insert((x1, y1), vertices.len());
                        graph.add_vertex((x1, y1));
                        vertices.len() - 1
                    }
                };
                let to = match vertices.get(&(x2, y2)) {
                    Some(i) => *i,
                    None => {
                        vertices.insert((x2, y2), vertices.len());
                        graph.add_vertex((x2, y2));
                        vertices.len() - 1
                    }
                };
                graph.add_edge(from, to, weight);
            }
        }
        line.clear();
    }

    // graph.reduce();
    Ok(graph)
}
