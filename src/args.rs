// Internals 
use crate::chars::args::CharsCommand;
use crate::hunts::args::HuntsCommand;

// Externals 
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct ChibiaArgs {

  #[clap(subcommand)]
  pub command: Chibia,

} 

#[derive(Debug, Subcommand)]
pub enum Chibia {
  
  /// Create, update, delete or list characters
  Chars(CharsCommand),

  /// Add, remove, list, or query hunt logs
  Hunts(HuntsCommand),
}

#[derive(Debug, Args)]
pub struct ShowArgs {

  /// ID of character to show (omit to show all)
  #[clap(long, default_value_t=0)]
  pub id: u32,

}