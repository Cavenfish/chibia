use std::ffi::OsStr;
use std::io;
use std::path::PathBuf;
use std::str::FromStr;
use std::{fmt, fs};

use crate::chars::args::CharInfo;
use crate::hunts::parse::CountedThing;
use crate::style::TibiaStyle;

use dirs::data_dir;
use rusqlite::{Connection, Error, Params};
use serde::{Deserialize, Serialize};

pub fn input<T: FromStr>(prompt: &str) -> Result<T, <T as FromStr>::Err> {
    let mut input = String::with_capacity(64);

    println!("{}", prompt);

    io::stdin()
        .read_line(&mut input)
        .expect("Input could not be read");

    input.trim().parse()
}

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

#[derive(Debug, Serialize, Deserialize)]
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
    pub hunt_start: String,
    pub hunt_end: String,
    pub hunt_length: String,
}

impl fmt::Display for FullHunt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.char_at_hunt)?;
        writeln!(f, "Hunt Info:")?;
        writeln!(f, "   ID:\t\t\t{}", self.id)?;
        writeln!(f, "   Start Date:\t\t{}", self.hunt_start)?;
        writeln!(f, "   End Date:\t\t{}", self.hunt_end)?;
        writeln!(f, "   Duration:\t\t{}", self.hunt_length)?;
        writeln!(f, "   Spawn:\t\t{}", &self.spawn)?;
        writeln!(f, "   Loot Multiplier:\t{}", self.loot_mult)?;
        writeln!(f, "   Loot:\t\t{}", self.loot.tibia())?;
        writeln!(f, "   Supplies:\t\t{}", self.supplies.tibia())?;
        writeln!(f, "   Balance:\t\t{}", self.balance.tibia())?;
        writeln!(
            f,
            "   Raw XP:\t\t{} ({}/h)",
            self.raw_xp.tibia(),
            self.raw_xp_h.tibia()
        )?;
        writeln!(
            f,
            "   XP:\t\t\t{} ({}/h)",
            self.xp.tibia(),
            self.xp_h.tibia()
        )?;
        writeln!(
            f,
            "   Damage:\t\t{} ({}/h)",
            self.damage.tibia(),
            self.damage_h.tibia()
        )?;
        writeln!(
            f,
            "   Healing:\t\t{} ({}/h)",
            self.healing.tibia(),
            self.healing_h.tibia()
        )?;

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

pub fn get_hunt(db: &Connection, id: u32) -> Result<FullHunt, Error> {
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
                spawn: row.get(1)?,
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
                loot_mult: row.get(13)?,
                hunt_start: row.get(14)?,
                hunt_end: row.get(15)?,
                hunt_length: row.get(16)?,
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
    pub xp: f64,
}

impl fmt::Display for HuntPreview {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{: <5} {: <15} {: <10} {: <10} {: <10}",
            self.id,
            &self.char_name,
            &self.balance.tibia(),
            self.raw_xp_h.tibia(),
            self.xp.tibia()
        )
    }
}

impl HuntPreview {
    pub fn print_header() {
        println!(
            "{: <5} {: <15} {: <10} {: <10} {: <10}",
            "ID", "Character", "Balance", "Raw XP/h", "Total XP"
        );

        println!("{:-<55}", "");
    }
}

pub fn get_all_hunts(db: &Connection) -> Result<Vec<HuntPreview>, Error> {
    let mut stmt = db.prepare(
        "SELECT a.id, b.name, a.balance, a.raw_xp_h, a.xp
        FROM hunts AS a 
        JOIN char_at_hunt AS b ON b.hunt_id = a.id",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(HuntPreview {
            id: row.get(0)?,
            char_name: row.get(1)?,
            balance: row.get(2)?,
            raw_xp_h: row.get(3)?,
            xp: row.get(4)?,
        })
    })?;

    rows.collect::<Result<Vec<HuntPreview>, _>>()
}

pub enum HuntChar {
    ID(u32),
    Struct(CharInfo),
}

pub fn insert_hunt<T: Params>(
    db: &Connection,
    hunt_params: T,
    hunt_char: HuntChar,
    mobs: &Vec<CountedThing>,
    items: &Vec<CountedThing>,
) -> Result<(), Error> {
    // Add data into hunts table
    db.execute(
        "INSERT INTO hunts (
        spawn, balance, damage, damage_h,
        healing, healing_h, loot, raw_xp, raw_xp_h,
        supplies, xp, xp_h, loot_mult, hunt_start,
        hunt_end, hunt_length
        ) VALUES (
        ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8,
        ?9, ?10, ?11, ?12, ?13, ?14, ?15,
        ?16
        )",
        hunt_params,
    )?;

    let id: u32 = db.last_insert_rowid().try_into().unwrap();

    // Add data to char_at_hunt
    match hunt_char {
        HuntChar::ID(char_id) => db.execute(
            "INSERT INTO char_at_hunt (
            hunt_id, name, vocation, level, magic,
            fist, sword, axe, club, distance, shielding
            ) SELECT ?1, name, vocation, level, magic,
            fist, sword, axe, club, distance, shielding
            FROM chars WHERE id = ?2",
            (id, char_id),
        )?,

        HuntChar::Struct(char_info) => db.execute(
            "INSERT INTO char_at_hunt (
            hunt_id, name, vocation, level, magic,
            fist, sword, axe, club, distance, shielding
            ) SELECT ?1, ?2, ?3, ?4, ?5, ?6, ?7, 
            ?8, ?9, ?10, ?11",
            (
                char_info.id,
                char_info.name,
                char_info.vocation,
                char_info.level,
                char_info.ml,
                char_info.fl,
                char_info.sl,
                char_info.al,
                char_info.cl,
                char_info.dl,
                char_info.shl,
            ),
        )?,
    };

    // Add data into mob_kills table
    for mob in mobs {
        db.execute(
            "INSERT INTO mob_kills (
            hunt_id, count, name
            ) VALUES (?1, ?2, ?3)",
            (id, mob.count, &mob.name),
        )?;
    }

    // Add data into items_looted table
    for item in items {
        db.execute(
            "INSERT INTO items_looted (
            hunt_id, count, name
            ) VALUES (?1, ?2, ?3)",
            (id, item.count, &item.name),
        )?;
    }

    Ok(())
}
