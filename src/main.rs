mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;
mod d9;

mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;

mod read;

fn main() {
  let args: Vec<String> = std::env::args().collect();

  if !(3 <= args.len() && args.len() <= 4) {
    panic!("incorrect usage");
  }

  let input_file = format!(
    "input/{}{}.txt",
    args[1],
    args.get(3).unwrap_or(&String::new())
  );

  let solution = match args[1].as_str() {
    "1" => match args[2].as_str() {
      "a" => d1::a::main(crate::read::get_reader(&input_file)),
      "b" => d1::b::main(crate::read::get_reader(&input_file)),
      _ => panic!("solution not found"),
    },
    "2" => match args[2].as_str() {
      "a" => d2::a::main(crate::read::get_reader(&input_file)),
      "b" => d2::b::main(crate::read::get_reader(&input_file)),
      _ => panic!("solution not found"),
    },
    "3" => match args[2].as_str() {
      "a" => d3::a::main(crate::read::get_reader(&input_file)),
      "b" => d3::b::main(input_file),
      _ => panic!("solution not found"),
    },
    "4" => match args[2].as_str() {
      "a" => d4::a::main(input_file),
      "b" => d4::b::main(input_file),
      _ => panic!("solution not found"),
    },
    "5" => match args[2].as_str() {
      "a" => d5::a::main(input_file),
      "b" => d5::b::main(input_file),
      _ => panic!("solution not found"),
    },
    "6" => match args[2].as_str() {
      "a" => d6::a::main(input_file),
      "b" => d6::b::main(input_file),
      _ => panic!("solution not found"),
    },
    "7" => match args[2].as_str() {
      "a" => d7::a::main(input_file),
      "b" => d7::b::main(input_file),
      _ => panic!("solution not found"),
    },
    "8" => match args[2].as_str() {
      "a" => d8::a::main(input_file),
      "b" => d8::b::main(input_file),
      _ => panic!("solution not found"),
    },
    "9" => match args[2].as_str() {
      "a" => d9::a::main(input_file),
      "b" => d9::b::main(input_file),
      _ => panic!("solution not found"),
    },
    "10" => match args[2].as_str() {
      "a" => d10::a::main(input_file),
      "b" => d10::b::main(input_file),
      _ => panic!("solution not found"),
    },
    "11" => match args[2].as_str() {
      "a" => d11::a::main(input_file),
      "b" => d11::b::main(input_file),
      _ => panic!("solution not found"),
    },
    "12" => match args[2].as_str() {
      "a" => d12::a::main(input_file),
      "b" => d12::b::main(input_file),
      _ => panic!("solution not found"),
    },
    "13" => match args[2].as_str() {
      "a" => d13::a::main(input_file),
      "b" => d13::b::main(input_file),
      _ => panic!("solution not found"),
    },
    "14" => match args[2].as_str() {
      "a" => d14::a::main(input_file),
      "b" => d14::b::main(input_file),
      _ => panic!("solution not found"),
    },
    "15" => match args[2].as_str() {
      "a" => d15::a::main(input_file),
      "b" => d15::b::main(input_file),
      _ => panic!("solution not found"),
    },
    "16" => match args[2].as_str() {
      "a" => d16::a::main(input_file),
      "b" => d16::b::main(input_file),
      _ => panic!("solution not found"),
    },
    _ => panic!("solution not found"),
  };

  println!("Solution for {}.{} is... {}", args[1], args[2], solution,);
}
