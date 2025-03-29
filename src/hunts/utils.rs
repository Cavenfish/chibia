use std::fs;
use std::ffi::OsStr;
use crate::db::load_db;
use crate::hunts::parse::{read_hunt_json, HuntInfo};

use dirs::data_dir;
use rusqlite::{Error};

pub fn prep_hunt_logs() {

  let tibia = data_dir().unwrap()
    .join("CipSoft GmbH/Tibia/packages/Tibia/log");

  let json = &OsStr::new("json");

  let json_files = Vec::from_iter(
    fs::read_dir(&tibia).unwrap()
      .filter_map(Result::ok)
      .map(|e| e.path())
      .filter(|e| e.extension() == Some(json)),
  );

  let tmpdir = data_dir().unwrap()
    .join("chibia/data/pending");

  for i in 0..json_files.len() {
    let j = format!("{}.json", i);
    let f = tmpdir.join(&j);

    println!("{:?}", f);
    println!("{:?}", json_files[i]);

    fs::rename(&json_files[i], &f);

    let d = read_hunt_json(&f);

    println!("Pending Hunt {}:", i);
    println!("{:#?}", d);
  }
}

// pub fn get_hunt(id: u32) -> Result<HuntInfo, Error> {

// }

// pub fn get_all_hunts() -> Result<Vec<HuntInfo>, Error> {

// }