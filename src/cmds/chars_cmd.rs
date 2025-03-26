use crate::db::load_db;

pub fn handle_chars_cmd(cmd: CharsCommand) {

  match cmd.command {
    CharsSubcommand::Add(cmd) => add_char(cmd),
    CharsSubcommand::LevelUp(cmd) => level_up_char(cmd),
    CharsSubcommand::SkillUp(cmd) => skill_up_char(cmd),
    CharsSubcommand::Delete(cmd) => delete_char(cmd),
    CharsSubcommand::Show => show_chars(),
  }

}

pub fn add_char(cmd: CharInfo) {
  let db = load_db.expect("Failed to load DB");

  db.execute(
    "INSERT INTO chars (
      name, vocation, level, magic_level, 
      fist_level, sword_level, axe_level,
      club_level, distance_level, shielding_level
    ) values (
      ?1, ?2, ?3, ?4, ?5,
      ?6, ?7, ?8, ?9, ?10
    )", 
    (&cmd.name, &cmd.vocation, cmd.level,
     cmd.ml, cmd.fl, cmd.sl, cmd.al, cmd.cl,
     cmd.dl, cmd.shl)
  ).expect("Failed to add character to DB");
  
}

pub fn level_up_char() {

}

pub fn skill_up_char() {

}

pub fn delete_char() {

}

pub fn handle_char_show(cmd: ShowCharArgs) {

  match cmd.id {

    // Show all case
    0 => show_chars(),

    // Show specific char
    _ => show_char(cmd.id),

  };

}

pub fn show_char(id: u32) {
  let character = get_char(id)
    .expect("Failed to find character in DB");

  println!("{:#?}", character);
}

pub fn show_chars() {

}