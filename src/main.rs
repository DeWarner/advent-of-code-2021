mod d1;
mod d2;
mod d3;
mod d4;
mod d5;

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

  let input = crate::read::get_reader(&input_file);

  let solution = match args[1].as_str() {
    "1" => match args[2].as_str() {
      "a" => d1::a::main(input),
      "b" => d1::b::main(input),
      _ => panic!("solution not found"),
    },
    "2" => match args[2].as_str() {
      "a" => d2::a::main(input),
      "b" => d2::b::main(input),
      _ => panic!("solution not found"),
    },
    "3" => match args[2].as_str() {
      "a" => d3::a::main(input),
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
    _ => panic!("solution not found"),
  };

  println!("Solution for {}.{} is... {}", args[1], args[2], solution,);
}
