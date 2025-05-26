use std::fs;
use std::fs::File;
use std::io::BufReader;

use crate::args::{ImpExArgs, ShowArgs};
use crate::chars::utils::get_char_id;
use crate::db::{SQLite, load_db};
use crate::hunts::args::{AddHunt, HuntsCommand, HuntsSubcommand, TopHunt, UpdateHunt};
use crate::hunts::parse::read_hunt_json;
use crate::hunts::utils::{
    HuntChar, HuntPreview, get_all_hunts, get_hunt, get_hunt_logs, input, insert_hunt,
};

use rusqlite::{Connection, named_params, params};
use serde_json::from_reader;

use super::utils::FullHunt;

pub fn handle_hunts_cmd(cmd: HuntsCommand) {
    let db = load_db().expect("Failed to load DB");

    match cmd.command {
        HuntsSubcommand::Add(cmd) => add_hunts(cmd, &db),
        HuntsSubcommand::Delete(cmd) => cmd.execute(&db).unwrap(),
        HuntsSubcommand::Update(cmd) => update_hunt(cmd, &db),
        HuntsSubcommand::Top(cmd) => top_hunt(cmd, &db),
        HuntsSubcommand::Show(cmd) => handle_hunt_show(&db, cmd),
        HuntsSubcommand::Export(cmd) => handle_hunt_export(&db, cmd),
        HuntsSubcommand::Import(cmd) => handle_hunt_import(&db, cmd),
    }
}

fn add_hunts(cmd: AddHunt, db: &Connection) {
    let logs = get_hunt_logs();
    let mut extra = cmd.clone();

    for log in logs {
        let info = read_hunt_json(&log);

        info.print_preview();

        extra.ask_and_update(&cmd);

        if !cmd.no_skip {
            let skip: String = input("skip?").unwrap();

            match skip.as_str() {
                "y" | "Y" | "yes" | "Yes" | "YES" => continue,
                "true" | "True" | "TRUE" => continue,
                _ => (),
            }
        }

        let hunt_params = params![
            &extra.spawn,
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
            extra.loot_mult,
            &info.hunt_start,
            &info.hunt_end,
            &info.hunt_length,
        ];

        insert_hunt(
            &db,
            hunt_params,
            HuntChar::ID(extra.id),
            &info.killed_monsters,
            &info.looted_items,
        )
        .unwrap();

        fs::remove_file(log).expect("Failed to delete file");
    }
}

fn update_hunt(cmd: UpdateHunt, db: &Connection) {
    db.execute(
        "UPDATE hunts SET spawn = ?1 WHERE id = ?2",
        (&cmd.spawn, cmd.id),
    )
    .expect("Failed to update hunt");
}

fn top_hunt(cmd: TopHunt, db: &Connection) {
    let id: u32 = get_char_id(&cmd.name, db).unwrap();

    let join_line = match id {
        0 => "JOIN chars AS b ON b.id = a.char_id",
        _ => "JOIN chars AS b ON b.id = :id",
    };

    let where_line = match (id, cmd.spawn.as_str()) {
        (0, "") => "",
        (0, _) => "WHERE a.spawn = :spawn",
        (_, "") => "WHERE a.char_id = :id",
        (_, _) => "WHERE (a.char_id = :id AND a.spawn = :spawn)",
    };

    let order_line = match (cmd.loot, cmd.xp) {
        (true, false) => "ORDER BY balance DESC",
        (false, true) => "ORDER BY raw_xp_h DESC",
        _ => panic!("Bad inputs"),
    };

    let sql = format!(
        "SELECT a.id, b.name, a.balance, a.raw_xp_h, a.xp
        FROM hunts AS a 
        {join_line}
        {where_line}
        {order_line}
        LIMIT :limit"
    );

    let params = match (sql.contains(":id"), sql.contains(":spawn")) {
        (true, false) => named_params! {":id": id, ":limit": cmd.limit},
        (false, true) => named_params! {":spawn": cmd.spawn, ":limit": cmd.limit},
        (true, true) => named_params! {":id": id, ":spawn": cmd.spawn, ":limit": cmd.limit},
        (false, false) => named_params! {":limit": cmd.limit},
    };

    let mut stmt = db.prepare(&sql).unwrap();

    let rows = stmt
        .query_map(params, |row| {
            Ok(HuntPreview {
                id: row.get(0)?,
                char_name: row.get(1)?,
                balance: row.get(2)?,
                raw_xp_h: row.get(3)?,
                xp: row.get(4)?,
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

fn handle_hunt_show(db: &Connection, cmd: ShowArgs) {
    match cmd.id {
        // Show all case
        0 => {
            let hunts = get_all_hunts(db).unwrap();

            HuntPreview::print_header();

            for row in hunts {
                println!("{}", row);
            }
        }

        // Show specific char
        _ => {
            let hunt = get_hunt(db, cmd.id).unwrap();

            println!("{}", hunt);
        }
    };
}

fn handle_hunt_export(db: &Connection, cmd: ImpExArgs) {
    if cmd.id != 0 {
        let hunt = get_hunt(db, cmd.id).unwrap();

        cmd.write_file(&hunt);
    } else {
        let max_id: u32 = db
            .query_row("SELECT MAX(id) FROM hunts", [], |row| row.get(0))
            .unwrap();

        let hunts: Vec<FullHunt> = (1..max_id + 1)
            .map(|id| get_hunt(db, id).unwrap())
            .collect();

        cmd.write_file(&hunts);
    }
}

fn handle_hunt_import(db: &Connection, cmd: ImpExArgs) {
    let f = File::open(&cmd.filename).unwrap();
    let reader = BufReader::new(f);

    let hunts: Vec<FullHunt> = from_reader(reader).unwrap();

    for hunt in hunts {
        let hunt_params = params![
            &hunt.spawn,
            hunt.balance,
            hunt.damage,
            hunt.damage_h,
            hunt.healing,
            hunt.healing_h,
            hunt.loot,
            hunt.raw_xp,
            hunt.raw_xp_h,
            hunt.supplies,
            hunt.xp,
            hunt.xp_h,
            hunt.loot_mult,
            &hunt.hunt_start,
            &hunt.hunt_end,
            &hunt.hunt_length,
        ];

        insert_hunt(
            db,
            hunt_params,
            HuntChar::Struct(hunt.char_at_hunt),
            &hunt.killed_monsters,
            &hunt.looted_items,
        )
        .unwrap();
    }
}
