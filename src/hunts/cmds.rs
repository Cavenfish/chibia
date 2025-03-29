use crate::db::load_db;
use crate::args::ShowArgs;
use crate::hunts::utils::prep_hunt_logs;
// use crate::hunts::utils::{get_all_hunts, get_hunt};
use crate::hunts::args::{
  HuntsCommand, HuntsSubcommand, AddHunt,
  DeleteHunt, TopHunt, 
};

pub fn handle_hunts_cmd(cmd: HuntsCommand) {

  match cmd.command {

    HuntsSubcommand::Prep => prep_hunt_logs(),
    HuntsSubcommand::Add(cmd) => add_hunt(cmd),
    HuntsSubcommand::Delete(cmd) => todo!(),//delete_hunt(cmd),
    HuntsSubcommand::Top(cmd) => todo!(),//top_hunt(cmd),
    HuntsSubcommand::Show(cmd) => handle_hunt_show(cmd),

  }
}

pub fn add_hunt(cmd: AddHunt) {

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
  // let chars = get_all_hunts().expect("Failed to query DB");

  // println!(
  //   "{: <5} {: <15} {: <10} {: <10}",
  //   "ID", "Vocation", "Name", "Level"
  // );

  // println!("{:-<55}", "");

  // for row in chars {

  //   println!(
  //     "{: <5} {: <15} {: <10} {: <10}",
  //     row.id, &row.vocation, &row.name, row.level
  //   );

  // }

}