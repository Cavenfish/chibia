use std::fs;
use std::path::Path;

use dirs::data_dir;
use rusqlite::{Connection, Error};


fn maybe_make_path(path: &Path) {

  if !path.exists() {
    fs::create_dir_all(path)
      .expect("Unable to make local dirs");
  }

}

pub fn init_local() {
  let chibia = data_dir().unwrap().join("chibia");
  let logdir = chibia.join("logs/data");
  let tmpdir = chibia.join("logs/pending");
  
  maybe_make_path(&chibia);
  maybe_make_path(&logdir);
  maybe_make_path(&tmpdir);

  let db_file = chibia.join("main.db");

  if !Path::new(&db_file).exists()  {
    create_new_db().expect("Failed to make db file");
  }

}

pub fn load_db() -> Result<Connection, Error> {
  let db_file = data_dir().unwrap().join("chibia/main.db");

  let db = Connection::open(db_file)?;

  Ok(db)
}

pub fn create_new_db() -> Result<(), Error> {
  let db = load_db()?;

  db.execute(
    "CREATE TABLE IF NOT EXISTS chars (
      id        INTEGER PRIMARY KEY,
      name      TEXT,
      vocation  TEXT,
      level     INTEGER,
      magic     INTEGER,
      fist      INTEGER,
      sword     INTEGER,
      axe       INTEGER,
      club      INTEGER,
      distance  INTEGER,
      shielding INTEGER
    )", ()
  )?;

  db.execute(
    "CREATE TABLE IF NOT EXISTS hunts (
      id       INTEGER PRIMARY KEY,
      char_id  INTEGER,
      raw_xp_h REAL,
      FOREIGN KEY (char_id) REFERENCES chars (id)
    )", ()
  )?;

  Ok(())
}