use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde_json::from_reader;
use serde::{de, Serialize, Deserialize, Deserializer};


#[derive(Debug, Serialize, Deserialize)]
pub struct HuntInfo {

  #[serde(rename = "Balance")]
  pub balance: String,

  #[serde(rename = "Damage")]
  pub damage: String,

  #[serde(rename = "Damage/h")]
  pub damage_h: String,

  #[serde(rename = "Healing")]
  pub healing: String,

  #[serde(rename = "Healing/h")]
  pub healing_h: String,

  #[serde(rename = "Killed Monsters")]
  pub killed_monsters: Vec<CountedThing>,

  #[serde(rename = "Loot")]
  pub loot: String,

  #[serde(rename = "Looted Items")]
  pub looted_items: Vec<CountedThing>,

  #[serde(rename = "Raw XP Gain")]
  pub raw_xp_gain: String,

  #[serde(rename = "Raw XP/h")]
  pub raw_xp_h: String,

  #[serde(rename = "Supplies")]
  pub supplies: String,

  #[serde(rename = "XP Gain")]
  pub xp_gain: String,

  #[serde(rename = "XP/h")]
  pub xp_h: String,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountedThing {
  
  #[serde(rename = "Count")]
  pub count: u32,

  #[serde(rename = "Name")]
  pub name: String,
}

pub fn read_hunt_json(file: &Path) -> HuntInfo {

  let f = File::open(file).expect("Failed");
  let reader = BufReader::new(f);


  let d: HuntInfo = from_reader(reader).expect("Failed");

  d
}

fn de_from_str<'de, D>(deserializer: D) -> Result<i64, D::Error> {
  
}