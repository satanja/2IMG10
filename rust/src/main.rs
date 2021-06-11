mod geometry;
mod graph;
mod io;
mod reeb_graph;

use geometry::{smallest_disk, Disk};
use reeb_graph::ReebGraph;
use std::path::PathBuf;
use structopt::StructOpt;
use voronoi::DCEL;

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
        
}
