mod args;
mod chars;
mod db;
mod hunts;
mod style;

// Internal
use args::{Chibia, ChibiaArgs};
use chars::cmds::handle_chars_cmd;
use db::init_local;
use hunts::cmds::handle_hunts_cmd;

// External
use clap::Parser;

fn main() {
    init_local();

    let args = ChibiaArgs::parse();

    match args.command {
        Chibia::Chars(cmd) => handle_chars_cmd(cmd),
        Chibia::Hunts(cmd) => handle_hunts_cmd(cmd),
    };
}
