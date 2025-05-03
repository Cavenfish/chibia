use std::ffi::OsStr;
use std::path::PathBuf;
use std::{fmt, fs};

use crate::chars::args::CharInfo;
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
    pub char_at_hunt: CharInfo,
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
        let loot = tibia_style_floats(self.loot);
        let supplies = tibia_style_floats(self.supplies);
        let balance = tibia_style_floats(self.balance);
        let raw_xp = tibia_style_floats(self.raw_xp);
        let raw_xp_h = tibia_style_floats(self.raw_xp_h);
        let xp = tibia_style_floats(self.xp);
        let xp_h = tibia_style_floats(self.xp_h);
        let healing = tibia_style_floats(self.healing);
        let healing_h = tibia_style_floats(self.healing_h);
        let damage = tibia_style_floats(self.damage);
        let damage_h = tibia_style_floats(self.damage_h);

        writeln!(f, "{}", self.char_at_hunt)?;
        writeln!(f, "Hunt Info:")?;
        writeln!(f, "   ID:\t\t\t{}", self.id)?;
        writeln!(f, "   Spawn:\t\t{}", &self.spawn)?;
        writeln!(f, "   Loot Multiplier:\t{}", self.loot_mult)?;
        writeln!(f, "   Loot (mult):\t\t{}", loot)?;
        writeln!(f, "   Supplies:\t\t{}", supplies)?;
        writeln!(f, "   Balance:\t\t{}", balance)?;
        writeln!(f, "   Raw XP:\t\t{} ({}/h)", raw_xp, raw_xp_h)?;
        writeln!(f, "   XP:\t\t\t{} ({}/h)", xp, xp_h)?;
        writeln!(f, "   Damage:\t\t{} ({}/h)", damage, damage_h)?;
        writeln!(f, "   Healing:\t\t{} ({}/h)", healing, healing_h)?;

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

fn tibia_style_floats(x: f64) -> String {
    if x > 1e6 {
        format!("{:.1}kk", x / 1e6)
    } else if x > 1e3 {
        format!("{:.1}k", x / 1e3)
    } else {
        format!("{x}")
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

fn get_char_at_hunt(db: &Connection, id: u32) -> Result<CharInfo, Error> {
    let character: CharInfo = db.query_row(
        "SELECT * FROM char_at_hunt
        WHERE hunt_id = ?1",
        [id],
        |row| {
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
        },
    )?;

    Ok(character)
}

pub fn get_hunt(id: u32) -> Result<FullHunt, Error> {
    let db = load_db()?;

    let mobs = get_counted_obj(&db, id, "mob_kills")?;

    let items = get_counted_obj(&db, id, "items_looted")?;

    let char_info = get_char_at_hunt(&db, id)?;

    // Get full hunt info
    let hunt: FullHunt = db.query_row(
        "SELECT * FROM hunts
        WHERE id = ?1",
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
                char_at_hunt: char_info,
            })
        },
    )?;

    Ok(hunt)
}

pub struct HuntPreview {
    pub id: u32,
    pub char_name: String,
    pub balance: f64,
    pub raw_xp_h: f64,
}

impl fmt::Display for HuntPreview {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bal = tibia_style_floats(self.balance);
        let xp = tibia_style_floats(self.raw_xp_h);

        write!(
            f,
            "{: <5} {: <15} {: <10} {: <10}",
            self.id, &self.char_name, &bal, &xp
        )
    }
}

impl HuntPreview {
    pub fn print_header() {
        println!(
            "{: <5} {: <15} {: <10} {: <10}",
            "ID", "Character", "Balance", "Raw XP/h"
        );

        println!("{:-<55}", "");
    }
}

pub fn get_all_hunts() -> Result<Vec<HuntPreview>, Error> {
    let db = load_db()?;

    let mut stmt = db.prepare(
        "SELECT a.id, b.name, a.balance, a.raw_xp_h 
        FROM hunts AS a 
        JOIN chars AS b ON b.id = a.char_id",
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
