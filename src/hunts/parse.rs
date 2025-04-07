use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde_json::from_reader;
use serde::{de::Error, Serialize, Deserialize, Deserializer};


#[derive(Debug, Serialize, Deserialize)]
pub struct HuntInfo {

  #[serde(rename = "Balance", deserialize_with="de_from_str")]
  pub balance: i64,

  #[serde(rename = "Damage", deserialize_with="de_from_str")]
  pub damage: i64,

  #[serde(rename = "Damage/h", deserialize_with="de_from_str")]
  pub damage_h: i64,

  #[serde(rename = "Healing", deserialize_with="de_from_str")]
  pub healing: i64,

  #[serde(rename = "Healing/h", deserialize_with="de_from_str")]
  pub healing_h: i64,

  #[serde(rename = "Killed Monsters")]
  pub killed_monsters: Vec<CountedThing>,

  #[serde(rename = "Loot", deserialize_with="de_from_str")]
  pub loot: i64,

  #[serde(rename = "Looted Items")]
  pub looted_items: Vec<CountedThing>,

  #[serde(rename = "Raw XP Gain", deserialize_with="de_from_str")]
  pub raw_xp: i64,

  #[serde(rename = "Raw XP/h", deserialize_with="de_from_str")]
  pub raw_xp_h: i64,

  #[serde(rename = "Supplies", deserialize_with="de_from_str")]
  pub supplies: i64,

  #[serde(rename = "XP Gain", deserialize_with="de_from_str")]
  pub xp: i64,

  #[serde(rename = "XP/h", deserialize_with="de_from_str")]
  pub xp_h: i64,

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

fn de_from_str<'de, D>(deserializer: D) -> 
  Result<i64, D::Error> where D: Deserializer<'de> {

  let s: String = Deserialize::deserialize(deserializer)?;

  let f: i64 = s.replace(",", "").parse()
    .expect("Must be valid number");
  
  Ok(f)
}