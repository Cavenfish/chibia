use std::fs;

use crate::args::ShowArgs;
use crate::chars::utils::get_char_id;
use crate::db::{SQLite, load_db};
use crate::hunts::args::{AddHunt, HuntsCommand, HuntsSubcommand, TopHunt, UpdateHunt};
use crate::hunts::parse::read_hunt_json;
use crate::hunts::utils::{HuntPreview, get_all_hunts, get_hunt, get_hunt_logs, input};

use rusqlite::{Connection, named_params, params};

pub fn handle_hunts_cmd(cmd: HuntsCommand) {
    let db = load_db().expect("Failed to load DB");

    match cmd.command {
        HuntsSubcommand::Add(cmd) => add_hunts(cmd, &db),
        HuntsSubcommand::Delete(cmd) => cmd.execute(&db).unwrap(),
        HuntsSubcommand::Update(cmd) => update_hunt(cmd, &db),
        HuntsSubcommand::Top(cmd) => top_hunt(cmd, &db),
        HuntsSubcommand::Show(cmd) => handle_hunt_show(cmd),
    }
}

fn add_hunts(cmd: AddHunt, db: &Connection) {
    let logs = get_hunt_logs();
    let mut extra = cmd.clone();

    for log in logs {
        let info = read_hunt_json(&log);

        info.print_preview();

        extra.ask_and_update(&cmd);

        let skip: bool = input("skip?").unwrap();

        if skip {
            continue;
        }

        db.execute(
            "INSERT INTO hunts (
            char_id, spawn, balance, damage, damage_h,
            healing, healing_h, loot, raw_xp, raw_xp_h,
            supplies, xp, xp_h, loot_mult, hunt_start,
            hunt_end, hunt_length
            ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8,
            ?9, ?10, ?11, ?12, ?13, ?14, ?15,
            ?16, ?17
            )",
            params![
                extra.id,
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
            ],
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
            (id, extra.id),
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
