use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct CharsCommand {

  #[clap(subcommand)]
  pub command: CharsSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum CharsSubcommand {

  /// Create a new character
  Create(CreateChar),

  /// Update level of a character
  LevelUp(LevelUpChar),

  /// Update skill of a character
  SkillUp(SkillUpChar),

  /// Delete a character
  Delete(DeleteChar),

  /// List all characters
  Show,
}

#[derive(Debug, Args)]
pub struct CreateChar {
  
  /// Name 
  pub name: String,

  /// Vocation
  pub vocation: String,

  /// Level
  pub level: u16,

  /// Magic level
  pub ml: u8,

  /// Fist level
  pub fl: u8,

  /// Sword level
  pub sl: u8,

  /// Axe level
  pub al: u8,

  /// Club level
  pub cl: u8,

  /// Distance level
  pub dl: u8,

  /// Shielding level
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

  /// Name of character 
  pub name: String,

}