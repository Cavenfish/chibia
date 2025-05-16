use crate::args::ShowArgs;
use crate::db::SQLite;
use crate::hunts::utils::input;

use clap::{Args, Subcommand};
use rusqlite::{Connection, Error};

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

    /// Update a hunt log
    Update(UpdateHunt),

    /// Get top hunting spots
    Top(TopHunt),

    /// List all hunt logs
    Show(ShowArgs),
}

#[derive(Debug, Args, Clone)]
pub struct AddHunt {
    /// ID of character used on hunt
    #[clap(long, default_value_t = 0)]
    pub id: u32,

    /// Location of hunt
    #[clap(long, default_value = "Unknown")]
    pub spawn: String,

    /// Loot multiplier during hunt
    #[clap(long, default_value_t = 0.0)]
    pub loot_mult: f64,
}

impl AddHunt {
    pub fn ask_and_update(&mut self, og: &AddHunt) {
        if og.spawn == "Unknown" {
            self.spawn = input("Spwan name?").unwrap();
        };

        if og.id == 0 {
            self.id = input("Character ID?").unwrap();
        };

        if og.loot_mult == 0.0 {
            self.loot_mult = input("Loot multiplier?").unwrap();
        };
        println!();
    }
}

#[derive(Debug, Args)]
pub struct DeleteHunt {
    /// ID of hunt log to delete
    pub id: u32,
}

impl SQLite for DeleteHunt {
    fn execute(&self, db: &Connection) -> Result<(), Error> {
        db.execute("DELETE FROM mob_kills WHERE hunt_id = ?1", (self.id,))?;

        db.execute("DELETE FROM items_looted WHERE hunt_id = ?1", (self.id,))?;

        db.execute("DELETE FROM char_at_hunt WHERE hunt_id = ?1", (self.id,))?;

        db.execute("DELETE FROM hunts WHERE id = ?1", (self.id,))?;

        Ok(())
    }
}

#[derive(Debug, Args)]
pub struct UpdateHunt {
    /// ID of hunt log to update
    #[clap(short, long)]
    pub id: u32,

    /// New spawn name
    #[clap(short, long)]
    pub spawn: String,
}

#[derive(Debug, Args)]
pub struct TopHunt {
    /// Character used on hunts
    #[clap(long, default_value = "")]
    pub name: String,

    /// Restrict hunts to given spawn
    #[clap(long, default_value = "")]
    pub spawn: String,

    /// Sort by loot
    #[clap(long, action)]
    pub loot: bool,

    /// Sort by loot
    #[clap(long, action)]
    pub xp: bool,

    /// Number of hunts to show
    #[clap(long, default_value_t = 5)]
    pub limit: u32,
    // Minimum level
    // #[clap(long, default_value_t=1)]
    // pub min_level: u16,
}
