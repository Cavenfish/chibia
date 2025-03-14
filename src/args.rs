pub mod chars_args;
pub mod hunts_args;

// Internals 
use chars_args::CharsCommand;
use hunts_args::HuntsCommand;

// Externals 
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct ChibiaArgs {
  #[clap(subcommand)]
  pub entity_type: EntityType,
} 


// TODO: Add gear subcommand

#[derive(Debug, Subcommand)]
pub enum EntityType {
  
  /// Create, update, delete or list characters
  Chars(CharsCommand),

  /// Add, remove, list, or query hunt logs
  Hunts(HuntsCommand),
}


