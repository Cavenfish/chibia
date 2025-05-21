use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::style::TibiaStyle;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::from_reader;

#[derive(Debug, Serialize, Deserialize)]
pub struct HuntInfo {
    #[serde(rename = "Balance", deserialize_with = "de_from_str")]
    pub balance: f64,

    #[serde(rename = "Damage", deserialize_with = "de_from_str")]
    pub damage: f64,

    #[serde(rename = "Damage/h", deserialize_with = "de_from_str")]
    pub damage_h: f64,

    #[serde(rename = "Healing", deserialize_with = "de_from_str")]
    pub healing: f64,

    #[serde(rename = "Healing/h", deserialize_with = "de_from_str")]
    pub healing_h: f64,

    #[serde(rename = "Killed Monsters")]
    pub killed_monsters: Vec<CountedThing>,

    #[serde(rename = "Loot", deserialize_with = "de_from_str")]
    pub loot: f64,

    #[serde(rename = "Looted Items")]
    pub looted_items: Vec<CountedThing>,

    #[serde(rename = "Raw XP Gain", deserialize_with = "de_from_str")]
    pub raw_xp: f64,

    #[serde(rename = "Raw XP/h", deserialize_with = "de_from_str")]
    pub raw_xp_h: f64,

    #[serde(rename = "Supplies", deserialize_with = "de_from_str")]
    pub supplies: f64,

    #[serde(rename = "XP Gain", deserialize_with = "de_from_str")]
    pub xp: f64,

    #[serde(rename = "XP/h", deserialize_with = "de_from_str")]
    pub xp_h: f64,

    #[serde(rename = "Session start")]
    pub hunt_start: String,

    #[serde(rename = "Session end")]
    pub hunt_end: String,

    #[serde(rename = "Session length")]
    pub hunt_length: String,
}

impl HuntInfo {
    pub fn print_preview(&self) {
        println!("Total XP Gain: {}", self.xp.tibia());
        println!("Raw XP/h: {}", self.raw_xp_h.tibia());
        println!("Balance: {}", self.balance.tibia());

        let n = self.killed_monsters.len();

        println!("Preview Monsters Killed:");
        if n < 5 {
            for mob in &self.killed_monsters[0..n] {
                println!("   -- {} {}", mob.count, &mob.name);
            }
        } else {
            for mob in &self.killed_monsters[0..4] {
                println!("   -- {} {}", mob.count, &mob.name);
            }
        }
        println!();
    }
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

fn de_from_str<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    let f: f64 = s.replace(",", "").parse().expect("Must be valid number");

    Ok(f)
}
