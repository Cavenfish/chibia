use std::fs::File;
use std::io::BufReader;

use crate::args::{ImpExArgs, ShowArgs};
use crate::chars::args::{
    CharInfo, CharsCommand, CharsSubcommand, DeleteChar, LevelUpChar, SkillUpChar,
};
use crate::chars::utils::{get_all_chars, get_char};
use crate::db::load_db;

use serde_json::from_reader;

pub fn handle_chars_cmd(cmd: CharsCommand) {
    match cmd.command {
        CharsSubcommand::Add(cmd) => add_char(cmd),
        CharsSubcommand::LevelUp(cmd) => level_up_char(cmd),
        CharsSubcommand::SkillUp(cmd) => skill_up_char(cmd),
        CharsSubcommand::Delete(cmd) => delete_char(cmd),
        CharsSubcommand::Import(cmd) => import_chars(cmd),
        CharsSubcommand::Show(cmd) => handle_char_show(cmd),
        CharsSubcommand::Export(cmd) => {
            let chars = get_all_chars().expect("Failed to query DB");

            cmd.write_file(&chars);
        }
    }
}

fn add_char(cmd: CharInfo) {
    let db = load_db().expect("Failed to load DB");

    cmd.insert(&db).expect("Failed to add character to DB");
}

fn level_up_char(cmd: LevelUpChar) {
    let db = load_db().expect("Failed to load DB");

    db.execute(
        "UPDATE chars SET level = level + ?1 WHERE id = ?2",
        (cmd.n, cmd.id),
    )
    .expect("Failed to update character level");
}

fn skill_up_char(cmd: SkillUpChar) {
    let db = load_db().expect("Failed to load DB");

    let tmp = format!("UPDATE chars SET {0} = {0} + ?1 WHERE id = ?2", &cmd.skill);

    db.execute(&tmp, (cmd.n, cmd.id))
        .expect("Failed to update character level");
}

fn delete_char(cmd: DeleteChar) {
    let db = load_db().expect("Failed to load DB");

    db.execute("DELETE FROM chars WHERE id = ?1", (cmd.id,))
        .expect("Failed to delete character");
}

fn import_chars(cmd: ImpExArgs) {
    let db = load_db().expect("Failed to load DB");
    let f = File::open(&cmd.filename).expect("Failed");
    let reader = BufReader::new(f);

    let chars: Vec<CharInfo> = from_reader(reader).expect("Failed");

    for char in chars {
        char.insert(&db).expect("Failed to add character to DB");
    }
}

fn handle_char_show(cmd: ShowArgs) {
    match cmd.id {
        // Show all case
        0 => show_chars(),

        // Show specific char
        _ => show_char(cmd.id),
    };
}

fn show_char(id: u32) {
    let character = get_char(id).expect("Failed to find character in DB");

    println!("{}", character);
}

fn show_chars() {
    let chars = get_all_chars().expect("Failed to query DB");

    println!(
        "{: <5} {: <10} {: <15} {: >7}",
        "ID", "Vocation", "Name", "Level"
    );

    println!("{:-<45}", "");

    for row in chars {
        println!(
            "{: <5} {: <10} {: <15} {: >6}",
            row.id, &row.vocation, &row.name, row.level
        );
    }
}
