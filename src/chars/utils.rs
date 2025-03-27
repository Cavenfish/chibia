use crate::db::load_db;
use crate::args::chars_args::CharInfo;

use rusqlite::{Error};

pub fn get_char(id: u32) -> Result<CharInfo, Error> {
  let db = load_db()?;

  let query = format!(
    "SELECT * FROM rbal WHERE id = {}", id,
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