use clap::Parser;
use std::path::PathBuf;

use c_preprocessor::preprocess;

#[derive(Parser, Debug)]
#[command(bin_name="licc", version, about, long_about = None)]
struct Args {
    path: PathBuf,
}

fn main() {
    let args = Args::parse();

    let processed = preprocess(args.path);
    println!("{:?}", processed);
}
