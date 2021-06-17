mod geometry;
mod graph;
mod io;
mod reeb_graph;

use reeb_graph::ReebGraph;
use std::{fs::DirEntry, io::Error, path::PathBuf};
use structopt::StructOpt;

use crate::reeb_graph::CriticalPoint;

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
    // print_percentage(0, inputs.len());

    // let mut networks = Vec::new();
    // let mut island_stack = Vec::new();
    // for i in 0..inputs.len() {
    //     let path = inputs[i].as_ref().unwrap().path();
    //     let network = io::read_network(delta, &path).unwrap();
    //     let polygons = network.polygons();
    //     if polygons.len() != 0 {
    //         island_stack.push(polygons);
    //     // networks.push(network);
    //     print_percentage(i + 1, inputs.len());
    // }
    // println!("\n");

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
            compute_reeb_graph(inputs, delta, 0);
        }
        "disk" => {
            println!("Using the smallest enclosing disk centroid algorithm");
            compute_reeb_graph(inputs, delta, 1);
        }
        _ => println!("Algorithm not found."),
    }
}

fn compute_reeb_graph(inputs: Vec<Result<DirEntry, Error>>, delta: f64, method: i32) -> ReebGraph {
    let mut reeb = ReebGraph::new(&CriticalPoint::new(0));

    let mut old_islands = io::read_network(delta, &inputs[0].as_ref().unwrap().path())
        .unwrap()
        .polygons();

    let mut acc_ids = 1; // accumulated number of islands before old_islands
    let mut fails = 0;

    for i in 0..old_islands.len() {
        reeb.add_point(&CriticalPoint::new(0), &CriticalPoint::new(acc_ids + i));
        println!("Added {}", acc_ids + i);
    }

    acc_ids += old_islands.len();

    for layer in 1..inputs.len() {
        println!("{}", layer);
        let input = inputs[layer].as_ref();
        let islands = io::read_network(delta, &input.unwrap().path())
            .unwrap()
            .polygons();

        for poly_new in 0..islands.len() {
            let new_centroid = if method == 0 {
                islands[poly_new].centroid().unwrap()
            } else {
                islands[poly_new].smallest_disk_centroid().unwrap()
            };
            let mut placed = false;
            for poly_old in 0..old_islands.len() {
                let old_centroid = if method == 0 {
                    old_islands[poly_old].centroid().unwrap()
                } else {
                    old_islands[poly_old].smallest_disk_centroid().unwrap()
                };
                let old_contains_new = old_islands[poly_old].contains(&old_centroid); // if so: split or normal
                let new_contains_old = islands[poly_new].contains(&new_centroid); // if so: merge or normal
                if old_contains_new || new_contains_old {
                    placed = true;
                    reeb.add_point(
                        &CriticalPoint::new(acc_ids + poly_old),
                        &CriticalPoint::new(acc_ids + old_islands.len() + poly_new),
                    );
                }
            }
            if !placed {
                // oopsie
                //println!("Shit, I don't know how to connect {}", acc_ids + old_islands.len() + poly_new);
                reeb.add_point(
                    &CriticalPoint::new(0),
                    &CriticalPoint::new(acc_ids + old_islands.len() + poly_new),
                );
                fails += 1;
            }
        }
        acc_ids += old_islands.len();
        old_islands = islands;
    }
    acc_ids += old_islands.len();
    println!(
        "I managed to properly connect {}/{} nodes.",
        acc_ids - fails,
        acc_ids
    );

    return reeb;
}
