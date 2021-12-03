pub fn main(input: std::io::Lines<std::io::BufReader<std::fs::File>>) -> String {
  let mut depth: i32 = 0;
  let mut x: i32 = 0;
  for line in input {
    let line = line.expect("Could not read line");
    let tokens: Vec<&str> = line.split(" ").collect();
    if tokens.len() != 2 {
      panic!("badly formatted instruction: '{}'", line)
    }
    let scale = match tokens[1].parse::<i32>() {
      Ok(num) => num,
      Err(err) => panic!(
        "Invalid scale, could not parse into an integer '{}', {:?}",
        line, err
      ),
    };
    match tokens[0] {
      "up" => depth -= scale,
      "down" => depth += scale,
      "forward" => x += scale,
      instruction => panic!("invalid instruction: {}", instruction),
    };
  }
  format!(
    "Depth: {}, Horizontal pos: {}, answer: {}",
    depth,
    x,
    depth * x
  )
}
