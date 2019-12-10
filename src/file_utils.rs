use std::fs::File;
use std::io::prelude::*;

pub fn read(file_name: &str) -> String {
  let mut file = File::open(file_name).unwrap();
  let mut contents = String::new();

  file.read_to_string(&mut contents).unwrap();

  contents
}
