mod geometry;
mod graph;
mod io;
mod reeb_graph;

use geometry::{smallest_disk, Disk};
use reeb_graph::ReebGraph;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short, long, default_value = "100.")]
    delta: f64,

    #[structopt(short, long, default_value = "0")]
    start_time: usize,

    #[structopt(short, long, default_value = "662")]
    end_time: usize,

    #[structopt(short, long, parse(from_os_str), default_value = "networks/")]
    input_dir: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    let delta = opt.delta;
    let input_dir = opt.input_dir;

    let input_paths = std::fs::read_dir(input_dir.clone()).unwrap();

    let networks: Vec<_> = input_paths
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
        .map(|path| io::read_network(delta, &path.unwrap().path()).unwrap())
        .collect();

    let island_stack: Vec<_> = networks.into_iter().map(|g| g.polygons()).collect();

    println!("hello world!");
}
