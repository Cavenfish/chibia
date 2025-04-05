use crate::db::load_db;
use crate::args::ShowArgs;
use crate::hunts::utils::get_hunt_logs;
use crate::hunts::parse::read_hunt_json;
use crate::hunts::utils::{get_all_hunts};
use crate::hunts::args::{
  HuntsCommand, HuntsSubcommand, AddHunt,
  DeleteHunt, TopHunt, 
};

use dirs::data_dir;

pub fn handle_hunts_cmd(cmd: HuntsCommand) {

  match cmd.command {

    HuntsSubcommand::Add(cmd) => add_hunts(cmd),
    HuntsSubcommand::Delete(cmd) => todo!(),//delete_hunt(cmd),
    HuntsSubcommand::Top(cmd) => todo!(),//top_hunt(cmd),
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
        char_id, balance, damage, damage_h,
        healing, healing_h, loot, raw_xp, raw_xp_h,
        supplies, xp, xp_h
      ) values (
        ?1, ?2, ?3, ?4, ?5, ?6,
        ?7, ?8, ?9, ?10, ?11, ?12
      )", (
        cmd.id, info.balance, info.damage, info.damage_h,
        info.healing, info.healing_h, info.loot, info.raw_xp,
        info.raw_xp_h, info.supplies, info.xp, info.xp_h
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

  };


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
  // let character = get_hunt(id)
  //   .expect("Failed to find character in DB");

  // println!("{:#?}", character);
}

pub fn show_hunts() {
  let hunts = get_all_hunts().expect("Failed to query DB.");

  println!(
    "{: <5} {: <15} {: <10} {: <10}",
    "ID", "Char_id", "Balance", "Raw XP/h"
  );

  println!("{:-<55}", "");


  for row in hunts {

    println!(
      "{: <5} {: <15} {: <10} {: <10}",
      row.id, row.char_id, row.balance, row.raw_xp_h
    );

  };


}