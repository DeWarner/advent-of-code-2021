use std::io::prelude::*;

pub fn get_reader(path: &str) -> std::io::Lines<std::io::BufReader<std::fs::File>> {
  let file = std::fs::File::open(path).unwrap();
  std::io::BufReader::new(file).lines()
}
