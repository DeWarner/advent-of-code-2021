pub fn main(input: std::io::Lines<std::io::BufReader<std::fs::File>>) -> String {
  let mut previous_depth: Option<usize> = None;
  let mut increases = 0;
  for line in input {
    let line = line.expect("Could not read line from standard in");
    let new_depth = match line.parse::<usize>() {
      Ok(num) => num,
      Err(err) => panic!(
        "Invalid depth, could not parse into a positive integer '{}', {:?}",
        line, err
      ),
    };
    if let Some(x) = previous_depth {
      if new_depth > x {
        increases += 1;
      }
    }
    previous_depth = Some(new_depth);
  }
  format!("{}", increases)
}
