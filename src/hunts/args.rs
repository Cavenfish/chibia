use crate::args::ShowArgs;

use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct HuntsCommand {

  #[clap(subcommand)]
  pub command: HuntsSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum HuntsSubcommand {

  /// Add new hunt logs
  Add(AddHunt),

  /// Delete a hunt log
  Delete(DeleteHunt),

  /// Get top hunting spots
  Top(TopHunt),

  /// List all hunt logs
  Show(ShowArgs),
}

#[derive(Debug, Args)]
pub struct AddHunt {

  /// Character used on hunt
  #[clap(long, default_value="Uknown")]
  pub name: String,

  /// Location of hunt
  #[clap(long, default_value="Unknown")]
  pub spawn: String,

  /// Active loot prey
  #[clap(long, default_value_t=1.0)]
  pub prey: f32,

  /// Active loot charm
  #[clap(long, default_value_t=1.0)]
  pub charm: f32,

}

#[derive(Debug, Args)]
pub struct DeleteHunt {

  /// ID of hunt log to delete
  pub id: u32,

}

#[derive(Debug, Args)]
pub struct TopHunt {

  /// Character used on hunts
  pub name: String,

  /// Sort by
  pub sort: String,

  /// Minimum level
  pub min_level: u16,

}