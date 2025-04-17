use crate::args::ShowArgs;
use crate::db::load_db;

use clap::{Args, Subcommand};
use rusqlite::Error;

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

  // Character used on hunt
  // #[clap(short, long)]
  // pub name: String,

  /// ID of character used on hunt
  #[clap(short, long)]
  pub id: u32,

  /// Location of hunt
  #[clap(long, default_value="Unknown")]
  pub spawn: String,

  /// Loot multiplier during hunt
  #[clap(long, default_value_t=1.0)]
  pub loot_mult: f64,

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

  /// Sort by loot
  #[clap(long, action)]
  pub loot: bool,

  /// Sort by loot
  #[clap(long, action)]
  pub xp: bool,

  // Minimum level
  // #[clap(long, default_value_t=1)]
  // pub min_level: u16,

}

impl TopHunt {
  pub fn get_char_id(&self) -> Result<u32, Error> {
    let db = load_db()?;
    
    db.query_row(
      "SELECT id FROM chars WHERE name = ?1",
      [&self.name],
      |row| row.get(0),
    )
  }
}