use std::fs;

use dirs::data_dir;
use rusqlite::{Connection, Error};

pub trait SQLite {
    fn execute(&self, db: &Connection) -> Result<(), Error>;
}

pub fn init_local() {
    let chibia = data_dir().unwrap().join("chibia");

    if !chibia.exists() {
        fs::create_dir_all(&chibia).expect("Unable to make local dirs");
    }

    init_db().expect("Failed to make db file");
}

pub fn load_db() -> Result<Connection, Error> {
    let db_file = data_dir().unwrap().join("chibia/main.db");

    let db = Connection::open(db_file)?;

    Ok(db)
}

fn init_db() -> Result<(), Error> {
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
        shielding INTEGER)",
        (),
    )?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS hunts (
        id          INTEGER PRIMARY KEY,
        spawn       TEXT,
        balance     REAL,
        damage      REAL,
        damage_h    REAL,
        healing     REAL,
        healing_h   REAL,
        loot        REAL,
        raw_xp      REAL,
        raw_xp_h    REAL,
        supplies    REAL,
        xp          REAL,
        xp_h        REAL,
        loot_mult   REAL,
        hunt_start  TEXT,
        hunt_end    TEXT,
        hunt_length TEXT)",
        (),
    )?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS mob_kills (
        hunt_id INTEGER,
        count   INTEGER,
        name    TEXT,
        FOREIGN KEY (hunt_id) REFERENCES hunts (id))",
        (),
    )?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS items_looted (
        hunt_id INTEGER,
        count   INTEGER,
        name    TEXT,
        FOREIGN KEY (hunt_id) REFERENCES hunts (id))",
        (),
    )?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS char_at_hunt (
        hunt_id   INTEGER,
        name      TEXT,
        vocation  TEXT,
        level     INTEGER,
        magic     INTEGER,
        fist      INTEGER,
        sword     INTEGER,
        axe       INTEGER,
        club      INTEGER,
        distance  INTEGER,
        shielding INTEGER,
        FOREIGN KEY (hunt_id) REFERENCES hunts (id))",
        (),
    )?;

    Ok(())
}
