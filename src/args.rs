// Internals 
use chars::args::CharsCommand;
use hunts::args::HuntsCommand;

// Externals 
use clap::{Parser, Subcommand};

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
