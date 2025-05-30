use std::fs;
use std::io::Write;

// Internals
use crate::chars::args::CharsCommand;
use crate::hunts::args::HuntsCommand;

// Externals
use clap::{Args, Parser, Subcommand};
use serde::Serialize;
use serde_json::to_string_pretty;

#[derive(Debug, Parser)]
#[command(version, about, author)]
pub struct ChibiaArgs {
    #[clap(subcommand)]
    pub command: Chibia,
}

#[derive(Debug, Subcommand)]
pub enum Chibia {
    /// Manage character data
    Chars(CharsCommand),

    /// Manage hunt data
    Hunts(HuntsCommand),
}

#[derive(Debug, Args)]
pub struct ShowArgs {
    /// ID of item to show (omit to show all)
    #[clap(long, default_value_t = 0)]
    pub id: u32,
}

#[derive(Debug, Args)]
pub struct ImpExArgs {
    /// Filename
    #[clap(short, long)]
    pub filename: String,

    /// ID of item to export (omit for import)
    #[clap(long, default_value_t = 0)]
    pub id: u32,
}

impl ImpExArgs {
    pub fn write_file(&self, obj: &impl Serialize) {
        let msg = to_string_pretty(&obj).unwrap();

        let mut file = fs::File::create(&self.filename).expect("Unable to open file");

        write!(file, "{}", &msg).expect("Failed to write");
    }
}
