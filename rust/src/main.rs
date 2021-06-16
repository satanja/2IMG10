mod geometry;
mod graph;
mod io;
mod reeb_graph;

use fxhash::FxHashMap;
use geometry::{smallest_disk, Disk, Polygon};
use reeb_graph::ReebGraph;
use std::{io::Write, path::PathBuf};
use structopt::StructOpt;

use crate::reeb_graph::CriticalPoint;

/// Data structure that holds the critical point which will be used in the reeb graph,
/// and the list of polygons map to this critical point
struct Relation {
    critical_point: CriticalPoint,
    polygons: Vec<(usize, usize)>,
}

impl Relation {
    pub fn new(critical_point: CriticalPoint) -> Relation {
        Relation {
            critical_point,
            polygons: Vec::new(),
        }
    }

    pub fn add_polygon_index(&mut self, polygon: (usize, usize)) {
        self.polygons.push(polygon);
    }
}

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short, long, default_value = "1e7")]
    delta: f64,

    #[structopt(short, long, default_value = "0")]
    start_time: usize,

    #[structopt(short, long, default_value = "662")]
    end_time: usize,

    #[structopt(short, long, parse(from_os_str), default_value = "networks/")]
    input_dir: PathBuf,

    #[structopt(short, long, default_value = "0.")]
    x: f64,

    #[structopt(short, long, default_value = "0.")]
    y: f64,

    #[structopt(short, long, default_value = "centroid")]
    algorithm: String,
}

/// Simple percentage indicator for user feedback
fn print_percentage(progress: usize, total: usize) {
    let percentage = (progress as f64) / (total as f64) * 100.;
    print!("Processed {:.2}%\r", percentage);
    std::io::stdout().flush().unwrap();
}

/// Finds the polygon that contains `point`, if it exists, and returns its index in `islands`
fn point_location(islands: &Vec<Polygon>, point: &(f64, f64)) -> Option<usize> {
    for i in 0..islands.len() {
        let island = &islands[i];
        if island.contains(point) {
            return Some(i);
        }
    }
    None
}

fn main() {
    println!("Loading networks and constructing data structures...");
    let opt = Opt::from_args();

    let delta = opt.delta;
    let input_dir = opt.input_dir;
    let input_paths = std::fs::read_dir(input_dir).unwrap();
    let inputs: Vec<_> = input_paths
        .into_iter()
        .filter(|path| match path {
            Ok(dir_entry) => {
                let pb = dir_entry.path();
                match pb.extension() {
                    Some(extension) => extension == "txt",
                    _ => false,
                }
            }
            _ => false,
        })
        .collect();
    print_percentage(0, inputs.len());

    let mut networks = Vec::new();
    let mut island_stack = Vec::new();
    for i in 0..inputs.len() {
        let path = inputs[i].as_ref().unwrap().path();
        let network = io::read_network(delta, &path).unwrap();
        island_stack.push(network.polygons());
        networks.push(network);
        print_percentage(i + 1, inputs.len());
    }
    println!("\n");

    match opt.algorithm.as_ref() {
        "centroid" => {
            println!("Using the polygonal centroid algorithm");
        }
        "disk" => {
            println!("Using the smallest enclosing disk centroid algorithm");
        }
        _ => println!("Algorithm not found."),
    }
}
