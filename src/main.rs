mod db;
mod args;
mod chars;
mod hunts;

// Internal
use db::init_local;
use args::{ChibiaArgs, Chibia};
use chars::cmds::handle_chars_cmd;

// External 
use clap::Parser;


fn main() {

  init_local();

  let args = ChibiaArgs::parse();

  match args.command {
      Chibia::Chars(cmd) => handle_chars_cmd(cmd),
      Chibia::Hunts(_) => todo!()
      // Chibia::Hunts(cmd) => hunts_args(cmd),
  };
    
}