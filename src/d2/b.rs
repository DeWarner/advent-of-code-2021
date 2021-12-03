pub fn main(input: std::io::Lines<std::io::BufReader<std::fs::File>>) -> String {
  let mut aim: i32 = 0;
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
      "up" => aim -= scale,
      "down" => aim += scale,
      "forward" => {
        x += scale;
        depth += scale * aim;
      }
      instruction => panic!("invalid instruction: {}", instruction),
    };
  }
  format!(
    "aim: {}, depth: {}, Horizontal pos: {}, answer: {}",
    aim,
    depth,
    x,
    depth * x
  )
}
