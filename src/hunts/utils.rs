use std::fs;
use std::path::PathBuf;
use std::ffi::OsStr;
use crate::db::load_db;
use crate::hunts::parse::{read_hunt_json, HuntInfo};

use dirs::data_dir;
use rusqlite::{Error};

pub fn get_hunt_logs() -> Vec<PathBuf> {

  let tibia = data_dir().unwrap()
    .join("CipSoft GmbH/Tibia/packages/Tibia/log");

  let json = &OsStr::new("json");

  let json_files = Vec::from_iter(
    fs::read_dir(&tibia).unwrap()
      .filter_map(Result::ok)
      .map(|e| e.path())
      .filter(|e| e.extension() == Some(json)),
  );

  json_files
}

pub struct HuntPreview {

  // Hunt id
  pub id: u32,

  // Name of character used on hunt
  pub char_name: String,

  // Hunt balance
  pub balance: f64,

  // Hunt raw_xp_h
  pub raw_xp_h: f64
}

// pub fn get_hunt(id: u32) -> Result<HuntInfo, Error> {

// }

pub fn get_all_hunts() -> Result<Vec<HuntPreview>, Error> {
  let db = load_db()?;

  let mut stmt = db.prepare(
    "
      SELECT a.id, b.name, a.balance, a.raw_xp_h 
      FROM hunts AS a 
      JOIN chars AS b ON b.id = a.char_id
    "
  )?; 

  let rows = stmt.query_map([], |row| {
    Ok(HuntPreview {

      id: row.get(0)?,

      char_name: row.get(1)?,

      balance: row.get(2)?,

      raw_xp_h: row.get(3)?,
    })
  })?;

  let hunts = rows.collect::<Result<Vec<HuntPreview>, _>>();

  hunts
}