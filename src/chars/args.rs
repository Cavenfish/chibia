use std::fmt;
use crate::args::ShowArgs;

use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct CharsCommand {
  #[clap(subcommand)]
  pub command: CharsSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum CharsSubcommand {
  /// Create a new character
  Add(CharInfo),

  /// Update level of a character
  LevelUp(LevelUpChar),

  /// Update skill of a character
  SkillUp(SkillUpChar),

  /// Delete a character
  Delete(DeleteChar),

  /// List all characters
  Show(ShowArgs),
}

#[derive(Debug, Args)]
pub struct CharInfo {
  // rowid
  #[clap(skip)]
  pub id: u32,
  
  /// Name
  #[clap(short, long)]
  pub name: String,

  /// Vocation
  #[clap(short, long)]
  pub vocation: String,

  /// Level
  #[clap(short, long)]
  pub level: u16,

  /// Magic level
  #[clap(long, default_value_t=1)]
  pub ml: u8,

  /// Fist level
  #[clap(long, default_value_t=10)]
  pub fl: u8,

  /// Sword level
  #[clap(long, default_value_t=10)]
  pub sl: u8,

  /// Axe level
  #[clap(long, default_value_t=10)]
  pub al: u8,

  /// Club level
  #[clap(long, default_value_t=10)]
  pub cl: u8,

  /// Distance level
  #[clap(long, default_value_t=10)]
  pub dl: u8,

  /// Shielding level
  #[clap(long, default_value_t=10)]
  pub shl: u8,
}

impl fmt::Display for CharInfo {

  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "\n\t{}:\n", &self.name)?;
    write!(f, "\t\tLevel: {}\n", self.level)?;
    write!(f, "\t\tVocation: {}\n", &self.vocation)?;
    write!(f, "\t\tMagic Level: {}\n", self.ml)?;

    match self.vocation.as_str() {
      "Paladin" => write!(f, "\t\tDistance Fighting: {}\n", self.dl)?,
      "Monk" => write!(f, "\t\tFist Fighting: {}\n", self.fl)?,
      _ => (),
    };

    write!(f, "\t\tShielding Level: {}\n", self.shl)
  }

}


#[derive(Debug, Args)]
pub struct LevelUpChar {
  /// ID of character
  #[clap(short, long)]
  pub id: u32,

  /// Number of levels to add
  #[clap(short)]
  pub n: u8,
}

#[derive(Debug, Args)]
pub struct SkillUpChar {
  /// ID of character
  #[clap(short, long)]
  pub id: u32,

  /// Skill to level up
  #[clap(short, long)]
  pub skill: String,

  /// Number of levels to add
  #[clap(short)]
  pub n: u8,
}

#[derive(Debug, Args)]
pub struct DeleteChar {
  /// ID of character 
  #[clap(short, long)]
  pub id: u32,
}