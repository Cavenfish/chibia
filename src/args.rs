pub mod chars;
pub mod hunts;

// Internals 
use chars::CharsCommand;
use hunts::HuntsCommand;

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


