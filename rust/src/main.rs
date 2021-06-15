mod geometry;
mod graph;
mod io;
mod reeb_graph;

use geometry::{smallest_disk, Disk};
use reeb_graph::ReebGraph;
use std::{io::Write, path::PathBuf};
use structopt::StructOpt;

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

fn print_percentage(progress: usize, total: usize) {
    let percentage = (progress as f64) / (total as f64) * 100.;
    print!("processed {:.2}%\r", percentage);
    std::io::stdout().flush().unwrap();
}

fn main() {
    println!("loading networks and constructing data structures...");
    let opt = Opt::from_args();

    let delta = opt.delta;
    let input_dir = opt.input_dir;
    let input_paths = std::fs::read_dir(input_dir.clone()).unwrap();
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

    let mut island_stack = Vec::new();
    for i in 0..inputs.len() {
        let path = inputs[i].as_ref().unwrap().path();
        let network = io::read_network(delta, &path).unwrap();
        island_stack.push(network.polygons());
        print_percentage(i + 1, inputs.len());
    }
    println!();
    println!("completed!\n");

    match opt.algorithm.as_ref() {
        "centroid" => {
            print!("using the polygonal centroid algorithm!");
        }
        "disk" => {
            print!("using the smallest enclosing disk centroid algorithm!");
        }
        _ => println!("Algorithm not found."),
    }
}
