use std::ffi::OsStr;
use std::path::PathBuf;
use std::{fmt, fs};

use crate::db::load_db;
use crate::hunts::parse::CountedThing;

use dirs::data_dir;
use rusqlite::{Connection, Error};

pub fn get_hunt_logs() -> Vec<PathBuf> {
    let tibia = data_dir()
        .unwrap()
        .join("CipSoft GmbH/Tibia/packages/Tibia/log");

    let json = &OsStr::new("json");

    let json_files = Vec::from_iter(
        fs::read_dir(&tibia)
            .unwrap()
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|e| e.extension() == Some(json)),
    );

    json_files
}

#[derive(Debug)]
pub struct FullHunt {
    pub id: u32,

    pub char_name: String,

    pub spawn: String,

    pub balance: f64,

    pub damage: f64,

    pub damage_h: f64,

    pub healing: f64,

    pub healing_h: f64,

    pub killed_monsters: Vec<CountedThing>,

    pub loot: f64,

    pub looted_items: Vec<CountedThing>,

    pub raw_xp: f64,

    pub raw_xp_h: f64,

    pub supplies: f64,

    pub xp: f64,

    pub xp_h: f64,

    pub loot_mult: f64,
}

impl fmt::Display for FullHunt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "\nCharacter Info:")?;
        writeln!(f, "   Name: {}", &self.char_name)?;
        writeln!(f, "Hunt Info:")?;
        writeln!(f, "   ID: {}", self.id)?;
        writeln!(f, "   Spawn: {}", &self.spawn)?;
        writeln!(f, "   Loot (mult): {} ({})", self.loot, self.loot_mult)?;
        writeln!(f, "   Supplies: {}", self.supplies)?;
        writeln!(f, "   Balance: {}", self.balance)?;
        writeln!(f, "   Raw XP (/h): {} ({})", self.raw_xp, self.raw_xp_h)?;
        writeln!(f, "   XP (/h): {} ({})", self.xp, self.xp_h)?;
        writeln!(f, "   Damage (/h): {} ({})", self.damage, self.damage_h)?;
        writeln!(f, "   Healing (/h): {} ({})", self.healing, self.healing_h)?;

        writeln!(f, "Looted Items:")?;
        for item in &self.looted_items {
            writeln!(f, "   -- {} {}", item.count, &item.name)?;
        }

        writeln!(f, "Monsters Killed:")?;
        for mob in &self.killed_monsters {
            writeln!(f, "   -- {} {}", mob.count, &mob.name)?;
        }

        write!(f, "")
    }
}

fn get_counted_obj(db: &Connection, id: u32, table: &str) -> Result<Vec<CountedThing>, Error> {
    let query = format!("SELECT count, name FROM {0} WHERE hunt_id = {1}", table, id);

    let mut stmt = db.prepare(&query)?;

    let rows = stmt.query_map([], |row| {
        Ok(CountedThing {
            count: row.get(0)?,

            name: row.get(1)?,
        })
    })?;

    rows.collect::<Result<Vec<CountedThing>, _>>()
}

pub fn get_hunt(id: u32) -> Result<FullHunt, Error> {
    let db = load_db()?;

    let mobs = get_counted_obj(&db, id, "mob_kills")?;

    let items = get_counted_obj(&db, id, "items_looted")?;

    // Get full hunt info
    let hunt: FullHunt = db.query_row(
        "SELECT h.*, c.name FROM
        hunts AS h
        JOIN chars AS c ON c.id = h.char_id
        WHERE h.id = ?1",
        [id],
        |row| {
            Ok(FullHunt {
                id: row.get(0)?,

                spawn: row.get(2)?,

                balance: row.get(3)?,

                damage: row.get(4)?,

                damage_h: row.get(5)?,

                healing: row.get(6)?,

                healing_h: row.get(7)?,

                killed_monsters: mobs,

                loot: row.get(8)?,

                looted_items: items,

                raw_xp: row.get(9)?,

                raw_xp_h: row.get(10)?,

                supplies: row.get(11)?,

                xp: row.get(12)?,

                xp_h: row.get(13)?,

                loot_mult: row.get(14)?,

                char_name: row.get(15)?,
            })
        },
    )?;

    Ok(hunt)
}

pub struct HuntPreview {
    // Hunt id
    pub id: u32,

    // Name of character used on hunt
    pub char_name: String,

    // Hunt balance
    pub balance: f64,

    // Hunt raw_xp_h
    pub raw_xp_h: f64,
}

pub fn get_all_hunts() -> Result<Vec<HuntPreview>, Error> {
    let db = load_db()?;

    let mut stmt = db.prepare(
        "
        SELECT a.id, b.name, a.balance, a.raw_xp_h 
        FROM hunts AS a 
        JOIN chars AS b ON b.id = a.char_id
        ",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(HuntPreview {
            id: row.get(0)?,

            char_name: row.get(1)?,

            balance: row.get(2)?,

            raw_xp_h: row.get(3)?,
        })
    })?;

    rows.collect::<Result<Vec<HuntPreview>, _>>()
}
