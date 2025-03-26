mod db;
mod args;
mod cmds;
mod utils;

// Internal
use db::init_local;
use args::{ChibiaArgs, Chibia};

// External 
use clap::Parser;


fn main() {

  init_local();

  let args = ChibiaArgs::parse();

  // match args.command {
  //     Chibia::Chars(cmd) => handle_chars_cmd(cmd),
  //     Chibia::Hunts(cmd) => hunts_args(cmd),
  // };
    
}