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
  Show(ShowCharArgs),
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

#[derive(Debug, Args)]
pub struct LevelUpChar {

  /// Name of character 
  pub name: String,

  /// Number of levels to add
  pub n: u8,
}

#[derive(Debug, Args)]
pub struct SkillUpChar {

  /// Name of character 
  pub name: String,

  /// Skill to level up
  pub skill: String,

  /// Number of levels to add
  pub n_levels: u8,

}

#[derive(Debug, Args)]
pub struct DeleteChar {

  /// ID of character 
  pub id: u32,

}

#[derive(Debug, Args)]
pub struct ShowCharArgs {

  /// ID of character to show (omit to show all)
  pub id: u32,

}