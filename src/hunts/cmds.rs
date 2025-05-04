use std::fs;

use crate::args::ShowArgs;
use crate::db::load_db;
use crate::hunts::args::{AddHunt, DeleteHunt, HuntsCommand, HuntsSubcommand, TopHunt};
use crate::hunts::parse::read_hunt_json;
use crate::hunts::utils::{HuntPreview, get_all_hunts, get_hunt, get_hunt_logs};

use rusqlite::Connection;

pub fn handle_hunts_cmd(cmd: HuntsCommand) {
    let db = load_db().expect("Failed to load DB");

    match cmd.command {
        HuntsSubcommand::Add(cmd) => add_hunts(cmd, &db),
        HuntsSubcommand::Delete(cmd) => delete_hunt(cmd, &db),
        HuntsSubcommand::Top(cmd) => top_hunt(cmd, &db),
        HuntsSubcommand::Show(cmd) => handle_hunt_show(cmd),
    }
}

fn add_hunts(cmd: AddHunt, db: &Connection) {
    let logs = get_hunt_logs();

    for log in logs {
        let info = read_hunt_json(&log);

        db.execute(
            "INSERT INTO hunts (
            char_id, spawn, balance, damage, damage_h,
            healing, healing_h, loot, raw_xp, raw_xp_h,
            supplies, xp, xp_h, loot_mult
            ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, ?7, 
            ?8, ?9, ?10, ?11, ?12, ?13, ?14
            )",
            (
                cmd.id,
                &cmd.spawn,
                info.balance,
                info.damage,
                info.damage_h,
                info.healing,
                info.healing_h,
                info.loot,
                info.raw_xp,
                info.raw_xp_h,
                info.supplies,
                info.xp,
                info.xp_h,
                cmd.loot_mult,
            ),
        )
        .expect("Failed to insert into table");

        let id: u32 = db
            .last_insert_rowid()
            .try_into()
            .expect("Failed to convert id type");

        db.execute(
            "INSERT INTO char_at_hunt (
            hunt_id, name, vocation, level, magic,
            fist, sword, axe, club, distance, shielding
            ) SELECT ?1, name, vocation, level, magic,
            fist, sword, axe, club, distance, shielding
            FROM chars WHERE id = ?2",
            (id, cmd.id),
        )
        .expect("Failed to insert into table");

        for mob in info.killed_monsters {
            db.execute(
                "INSERT INTO mob_kills (
                hunt_id, count, name
                ) VALUES (?1, ?2, ?3)",
                (id, mob.count, &mob.name),
            )
            .expect("Failed to insert");
        }

        for item in info.looted_items {
            db.execute(
                "INSERT INTO items_looted (
                hunt_id, count, name
                ) VALUES (?1, ?2, ?3)",
                (id, item.count, &item.name),
            )
            .expect("Failed to insert");
        }

        fs::remove_file(log).expect("Failed to delete file");
    }
}

fn delete_hunt(cmd: DeleteHunt, db: &Connection) {
    db.execute("DELETE FROM mob_kills WHERE hunt_id = ?1", (cmd.id,))
        .expect("Failed to delete mobs");

    db.execute("DELETE FROM items_looted WHERE hunt_id = ?1", (cmd.id,))
        .expect("Failed to delete items");

    db.execute("DELETE FROM char_at_hunt WHERE hunt_id = ?1", (cmd.id,))
        .expect("Failed to delete items");

    db.execute("DELETE FROM hunts WHERE id = ?1", (cmd.id,))
        .expect("Failed to delete hunt");
}

fn top_hunt(cmd: TopHunt, db: &Connection) {
    let id: u32 = db
        .query_row("SELECT id FROM chars WHERE name = ?1", [&cmd.name], |row| {
            row.get(0)
        })
        .expect("Failed to get char id");

    let mut stmt = if cmd.loot && cmd.xp {
        panic!("Both --loot and --xp cannot be passed");
    } else if cmd.loot {
        db.prepare(
            "SELECT a.id, b.name, a.balance, a.raw_xp_h
            FROM hunts AS a 
            JOIN chars AS b ON b.id = ?1
            WHERE a.char_id = ?1
            ORDER BY balance DESC",
        )
        .expect("Failed to prepare query")
    } else if cmd.xp {
        db.prepare(
            "SELECT a.id, b.name, a.balance, a.raw_xp_h
            FROM hunts AS a 
            JOIN chars AS b ON b.id = ?1
            WHERE a.char_id = ?1
            ORDER BY raw_xp_h DESC",
        )
        .expect("Failed to prepare query")
    } else {
        panic!("Either --loot or --xp must be passed");
    };

    let rows = stmt
        .query_map([id], |row| {
            Ok(HuntPreview {
                id: row.get(0)?,
                char_name: row.get(1)?,
                balance: row.get(2)?,
                raw_xp_h: row.get(3)?,
            })
        })
        .expect("Failed to query DB");

    let hunts = rows
        .collect::<Result<Vec<HuntPreview>, _>>()
        .expect("Failed to collect hunts info");

    HuntPreview::print_header();

    for row in hunts {
        println!("{}", row);
    }
}

fn handle_hunt_show(cmd: ShowArgs) {
    match cmd.id {
        // Show all case
        0 => show_hunts(),

        // Show specific char
        _ => show_hunt(cmd.id),
    };
}

fn show_hunt(id: u32) {
    let hunt = get_hunt(id).expect("Failed to find hunt in DB");

    println!("{}", hunt);
}

fn show_hunts() {
    let hunts = get_all_hunts().expect("Failed to query DB.");

    HuntPreview::print_header();

    for row in hunts {
        println!("{}", row);
    }
}
