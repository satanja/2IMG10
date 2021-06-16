use std::fs::{read, File};
use std::io::Result;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use crate::graph::Graph;

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
    let mut vertices = Vec::with_capacity(n);

    for _ in 0..n {
        reader.read_line(&mut line).unwrap();
        let coords: Vec<_> = line.trim().split(' ').collect();
        let x = coords[1].parse::<i32>().unwrap();
        let y = coords[2].parse::<i32>().unwrap();

        vertices.push((x, y));
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
        let i = data_iter.next().unwrap().parse::<usize>().unwrap();
        let j = data_iter.next().unwrap().parse::<usize>().unwrap();
        let w = data_iter.next().unwrap();

        let weight = if w == "nan" {
            f64::NAN
        } else {
            w.parse::<f64>().unwrap()
        };
        if !weight.is_nan() && weight >= delta {
            let from = vertices[i];
            let to = vertices[j];

            graph.add_edge(from, to, weight);
        }
        line.clear();
    }

    // graph.reduce();
    Ok(graph)
}
