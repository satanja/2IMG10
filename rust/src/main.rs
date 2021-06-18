mod geometry;
mod graph;
mod io;
mod reeb_graph;

use crate::reeb_graph::CriticalPoint;
use fxhash::FxHashSet;
use reeb_graph::ReebGraph;
use std::collections::VecDeque;
use std::{fs::DirEntry, io::Error, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short, long, default_value = "1e2")]
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

fn main() {
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

    match opt.algorithm.as_ref() {
        "counting" => {
            println!("Counting the number of islands over time");
            println!("t,\t islands");
            for i in 0..inputs.len() {
                let input = inputs[i].as_ref();
                let network = io::read_network(delta, &input.unwrap().path()).unwrap();
                println!("{},\t {}", i, network.polygons().len());
            }
        }
        "centroid" => {
            println!("Using the polygonal centroid algorithm");
            compute_reeb_graph(inputs, delta, (opt.x, opt.y), 0);
        }
        "disk" => {
            println!("Using the smallest enclosing disk centroid algorithm");
            compute_reeb_graph(inputs, delta, (opt.x, opt.y), 1);
        }
        _ => println!("Algorithm not found."),
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct State {
    /// Id of the parent critical point
    parent_id: usize,

    /// The next layer to match for
    next_layer: usize,

    /// Index of the layer containing the polygon whose centroid we need to compute
    layer: usize,

    /// The index of the polygon in the layer
    index: usize,
}

fn compute_reeb_graph(
    inputs: Vec<Result<DirEntry, Error>>,
    delta: f64,
    start_point: (f64, f64),
    method: i32,
) -> ReebGraph {
    let mut reeb = ReebGraph::new(&CriticalPoint::new(0));
    let mut start_layer = 0;

    let mut found = false;
    let mut index = 0;

    let input = inputs[0].as_ref();
    let mut islands = io::read_network(delta, &input.unwrap().path())
        .unwrap()
        .polygons();

    'search_loop: for layer in 0..inputs.len() {
        for j in 0..islands.len() {
            let island = &islands[j];
            if island.contains(&start_point) {
                found = true;
                index = j;
                start_layer = layer;
                break 'search_loop;
            }
        }

        if layer + 1 < inputs.len() {
            let input = inputs[layer + 1].as_ref();
            islands = io::read_network(delta, &input.unwrap().path())
                .unwrap()
                .polygons();
        }
    }

    if !found {
        println!("Island containing {:?} not found", start_point);
        return reeb;
    }

    let mut queue = VecDeque::new();
    queue.push_back(State {
        parent_id: 0,
        next_layer: start_layer + 1,
        layer: start_layer,
        index,
    });

    let mut included = FxHashSet::default();
    included.insert(State {
        parent_id: 0,
        next_layer: start_layer + 1,
        layer: start_layer,
        index,
    });

    let mut old_layer = start_layer;
    let mut old_islands = islands;
    let mut new_layer = start_layer + 1;
    let next_input = inputs[start_layer + 1].as_ref();
    let mut new_islands = io::read_network(delta, &next_input.unwrap().path())
        .unwrap()
        .polygons();

    let mut start_ids = old_islands.len();

    while let Some(State {
        parent_id,
        next_layer,
        layer,
        index,
    }) = queue.pop_front()
    {
        if next_layer == inputs.len() {
            continue;
        }

        if layer != old_layer && next_layer != new_layer {
            start_ids += old_islands.len();
            old_layer = new_layer;
            old_islands = new_islands;

            new_layer = next_layer;
            let next_input = inputs[next_layer].as_ref();
            new_islands = io::read_network(delta, &next_input.unwrap().path())
                .unwrap()
                .polygons();
        }

        let old_centroid = if method == 0 {
            old_islands[index].centroid().unwrap()
        } else {
            old_islands[index].smallest_disk_centroid().unwrap()
        };

        for poly_new in 0..new_islands.len() {
            let new_centroid = if method == 0 {
                new_islands[poly_new].centroid().unwrap()
            } else {
                new_islands[poly_new].smallest_disk_centroid().unwrap()
            };

            let id = start_ids + poly_new;

            let old_contains_new = old_islands[index].contains(&new_centroid); // if so: split or normal
            let new_contains_old = new_islands[poly_new].contains(&old_centroid); // if so: merge or normal
            if old_contains_new || new_contains_old {
                reeb.add_point(&CriticalPoint::new(parent_id), &CriticalPoint::new(id));

                let state = State {
                    index: poly_new,
                    next_layer: new_layer + 1,
                    layer: new_layer,
                    parent_id: id,
                };

                if !included.contains(&state) {
                    included.insert(state.clone());
                    queue.push_back(state);
                }
            }
        }
    }

    reeb
}
