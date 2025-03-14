use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
  
  // Character name
  pub name: String,

  // Character vocation
  pub vocation: String,

  // Character level
  pub level: u16,

  // Character skills
  pub skills: CharSkills,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharSkills {
  
  // Magic level
  pub ml: u8,

  // Fist level
  pub fl: u8,

  // Sword level
  pub sl: u8,

  // Axe level
  pub al: u8,

  // Club level
  pub cl: u8,

  // Distance level
  pub dl: u8,

  // Shielding level
  pub shl: u8,
}