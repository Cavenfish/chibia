use crate::db::load_db;
use crate::chars::args::CharInfo;

use rusqlite::{Error};

pub fn get_char(id: u32) -> Result<CharInfo, Error> {
  let db = load_db()?;

  let query = format!(
    "SELECT * FROM chars WHERE id = {}", id,
  );

  let row: CharInfo = db.query_row(&query, [], |row| {
    Ok(CharInfo {

      id: row.get(0)?,

      name: row.get(1)?,

      vocation: row.get(2)?,

      level: row.get(3)?,

      ml: row.get(4)?,

      fl: row.get(5)?,

      sl: row.get(6)?,

      al: row.get(7)?,

      cl: row.get(8)?,

      dl: row.get(9)?,

      shl: row.get(10)?,
    })
  })?;

  Ok(row)
}

pub fn get_all_chars() -> Result<Vec<CharInfo>, Error> {
  let db = load_db()?;

  let mut stmt = db.prepare("SELECT * FROM chars")?;

  let tmp = stmt.query_map([], |row| {
    Ok(CharInfo {

      id: row.get(0)?,

      name: row.get(1)?,

      vocation: row.get(2)?,

      level: row.get(3)?,

      ml: row.get(4)?,

      fl: row.get(5)?,

      sl: row.get(6)?,

      al: row.get(7)?,

      cl: row.get(8)?,

      dl: row.get(9)?,

      shl: row.get(10)?,
    })
  })?;

  let chars = tmp.collect::<Result<Vec<CharInfo>, _>>();

  chars
}