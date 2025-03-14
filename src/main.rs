mod args;

use args::ChibiaArgs;
use clap::Parser;

fn main() {
    let args = ChibiaArgs::parse();
}
