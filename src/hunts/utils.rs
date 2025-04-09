use std::fs;
use std::path::PathBuf;
use std::ffi::OsStr;
use crate::db::load_db;
use crate::hunts::parse::{read_hunt_json, HuntInfo, CountedThing};

use dirs::data_dir;
use rusqlite::{Connection, Error};

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

fn get_counted_obj(db: &Connection, id: u32, table: &str) 
  -> Result<Vec<CountedThing>, Error> {
    
  let query = format!(
    "SELECT count, name FROM {0} WHERE hunt_id = {1}",
    table, id
  );

  let mut stmt = db.prepare(&query)?;

  let rows = stmt.query_map([], |row| {
    Ok(CountedThing {

      count: row.get(0)?,

      name: row.get(1)?

    })
  })?;

  rows.collect::<Result<Vec<CountedThing>, _>>()
}

pub fn get_hunt(id: u32) -> Result<HuntInfo, Error> {
  let db    = load_db()?;

  let mobs  = get_counted_obj(&db, id, "mob_kills")?;

  let items = get_counted_obj(&db, id, "items_looted")?;

  // Get full hunt info
  let info: HuntInfo = db.query_row(
    "SELECT * FROM hunts WHERE id = ?1", 
    [id,], |row| {
      Ok(HuntInfo {

        balance: row.get(2)?,

        damage: row.get(3)?,

        damage_h: row.get(4)?,

        healing: row.get(5)?,

        healing_h: row.get(6)?,

        killed_monsters: mobs,

        loot: row.get(7)?,

        looted_items: items,

        raw_xp: row.get(8)?,

        raw_xp_h: row.get(9)?,

        supplies: row.get(10)?,

        xp: row.get(11)?,

        xp_h: row.get(12)?,

      })
    }
  )?;
  
  Ok(info)
}

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