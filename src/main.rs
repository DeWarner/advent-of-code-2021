use std::io::prelude::*;

mod d1;

fn main() {
  let args: Vec<String> = std::env::args().collect();

  if args.len() != 3 {
    panic!("incorrect usage");
  }

  let input_file = format!("input/{}.txt", args[1]);
  let file = std::fs::File::open(input_file).unwrap();
  let input = std::io::BufReader::new(file).lines();

  let solution = match args[1].as_str() {
    "1" => match args[2].as_str() {
      "a" => d1::a::main(input),
      "b" => d1::b::main(input),
      _ => panic!("solution not found"),
    },
    _ => panic!("solution not found"),
  };

  println!("Solution for {}.{} is... {}", args[1], args[2], solution,);
}
