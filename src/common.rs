#![allow(dead_code)]
extern crate chrono;

use std::fs;
use std::path;
use std::error::Error;
use std::io::prelude::*;

pub fn backup_file_with_date(file_path: &str){
  if !path::Path::new(&file_path.to_string()).exists(){
    return;
  }

  let now = chrono::Utc::now().format("%F");
  let file_path_backup = format!("{}-{}", file_path, now);

  match fs::rename(file_path, file_path_backup){
    Err(why) => panic!("Error occured when rename the \"{}\" folder. [{}]", &file_path, why.description()),
    Ok(_) => (),
  }
}

pub fn read_string_from_path(file_path: &str) -> String{
  let mut f = fs::File::open(&file_path)
    .expect("file not found");
  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");
  contents
}

pub fn append_to_file(file_source: &mut fs::File, file_path: &str){
  let contents = read_string_from_path(&file_path);
  match file_source.write_all(contents.as_bytes()) {
    Err(why) => panic!("Error: couldn't append file {} [{}].", format!("{}", file_path), why.description()),
    Ok(_) => (),
  }

  // add line between two entries
  match file_source.write_all("\n".as_bytes()) {
    Err(_) => panic!("Error: couldn't write \n"),
    Ok(_) => (),
  }
}