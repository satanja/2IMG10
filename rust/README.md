# Island-Tracker

## Getting started with Rust
Download Rust [here](https://www.rust-lang.org/learn/get-started). You may be prompted to install C++ build tools, which is available [here](https://visualstudio.microsoft.com/visual-cpp-build-tools/).

## Running the tool
Firstly, make sure that in the current directory, i.e., `../2IMG10/rust/`, there is the directory `/networks/` containing all the computed networks of the TTGA tool. To run the tool, execute `cargo run --release` for the default configuration. 

### Command line options (Subject to change)
The tool supports several command line options, namely

* `-a`: the desired algorithm (default: polygonal centroid, see next subsection for a list of all the algorithms),
* `--start-time`: initial time to look for the island (default: `0`),
* `--delta` (default: `1e2`), note that scientific notation is supported,
* `--input-dir`: the input directory (default `./networks/`),
* `-x`: the x-coordinate of a point in the island you want to track,
* `-y`: the y-coordinate of a point the island you want to track.

To see the list of all the options, use `cargo run --release -- --help`. To use a specific option, e.g., selecting a different algorithm, use `cargo run --release -- -a disk`. Options can be combined in any order, e.g., `cargo run --release -- --delta 1000 -a disk`.

### Algorithms
Three algorithms are implemented, namely simply counting all the islands over each layer, tracking a specific island over time using centroids of islands (polygonal centroid algorithm), and tracking a specific island over time using centroids of smallest enclosing disks of islands (smallest enclosing disk algorithm).

To select the counting algorithm, use `cargo run --release -- -a counting`. To use the polygonal centroid algorithm, use `cargo run --release -- -a centroid`. To use the smallest enclosing disk centroid algorithm, use `cargo run --release -a disk`.


### Piping output (Subject to change)
It may be convenient to pipe the output of the tool to a file. On Windows, it is most convenient to use Command Prompt (cmd), i.e., use `cargo run --release [args] > output.txt`, this creates a file called `output.txt` containing all the output of the tool. 