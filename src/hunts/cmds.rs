use std::fs;
use crate::db::load_db;
use crate::args::ShowArgs;
use crate::hunts::utils::get_hunt_logs;
use crate::hunts::parse::read_hunt_json;
use crate::hunts::utils::{get_all_hunts, get_hunt};
use crate::hunts::args::{
  HuntsCommand, HuntsSubcommand, AddHunt,
  DeleteHunt, TopHunt, 
};

pub fn handle_hunts_cmd(cmd: HuntsCommand) {

  match cmd.command {

    HuntsSubcommand::Add(cmd) => add_hunts(cmd),
    HuntsSubcommand::Delete(cmd) => delete_hunt(cmd),
    HuntsSubcommand::Top(cmd) => top_hunt(cmd),
    HuntsSubcommand::Show(cmd) => handle_hunt_show(cmd),

  }
}

pub fn add_hunts(cmd: AddHunt) {
  let db = load_db().expect("Failed to load DB");

  let logs = get_hunt_logs();

  for log in logs {
    let info = read_hunt_json(&log);

    db.execute(
      "INSERT INTO hunts (
        char_id, spawn, balance, damage, damage_h,
        healing, healing_h, loot, raw_xp, raw_xp_h,
        supplies, xp, xp_h, loot_mult
      ) values (
        ?1, ?2, ?3, ?4, ?5, ?6, ?7, 
        ?8, ?9, ?10, ?11, ?12, ?13, ?14
      )", (
        cmd.id, &cmd.spawn, info.balance, info.damage, info.damage_h,
        info.healing, info.healing_h, info.loot, info.raw_xp,
        info.raw_xp_h, info.supplies, info.xp, info.xp_h, cmd.loot_mult
      )
    ).expect("Failed to insert into table");

    let id: u32 = db.last_insert_rowid()
      .try_into().expect("Failed to convert id type");

    for mob in info.killed_monsters {

      db.execute(
        "INSERT INTO mob_kills (
          hunt_id, count, name
        ) values (?1, ?2, ?3)",
        (id, mob.count, &mob.name)
      ).expect("Failed to insert");
      
    };

    for item in info.looted_items {

      db.execute(
        "INSERT INTO items_looted (
          hunt_id, count, name
        ) values (?1, ?2, ?3)",
        (id, item.count, &item.name)
      ).expect("Failed to insert");

    };

    fs::remove_file(log).expect("Failed to delete file");
  };


}

pub fn delete_hunt(cmd: DeleteHunt) {
  let db = load_db().expect("Failed to load DB");

  db.execute(
    "DELETE FROM mob_kills WHERE hunt_id = ?1", (cmd.id,)
  ).expect("Failed to delete mobs");

  db.execute(
    "DELETE FROM items_looted WHERE hunt_id = ?1", (cmd.id,)
  ).expect("Failed to delete items");

  db.execute(
    "DELETE FROM hunts WHERE id = ?1", (cmd.id,)
  ).expect("Failed to delete hunt");
}

pub fn top_hunt(cmd: TopHunt) {

  cmd.print_top_hunts().expect("fail");

}

pub fn handle_hunt_show(cmd: ShowArgs) {

  match cmd.id {

    // Show all case
    0 => show_hunts(),

    // Show specific char
    _ => show_hunt(cmd.id),

  };

}

pub fn show_hunt(id: u32) {
  let hunt = get_hunt(id).expect("Failed to find hunt in DB");

  println!("{:#?}", hunt);
}

pub fn show_hunts() {
  let hunts = get_all_hunts().expect("Failed to query DB.");

  println!(
    "{: <5} {: <15} {: <10} {: <10}",
    "ID", "Character", "Balance", "Raw XP/h"
  );

  println!("{:-<55}", "");


  for row in hunts {

    println!(
      "{: <5} {: <15} {: <10} {: <10}",
      row.id, &row.char_name, row.balance, row.raw_xp_h
    );

  };


}