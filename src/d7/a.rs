pub fn main(input_file: String) -> String {
  let mut crabs_opt: Option<Vec<i64>> = None;
  for line in crate::read::get_reader(&input_file) {
    let line = line.expect("Could not read line");
    println!("line:; '{}'", line);
    crabs_opt = Some(
      line
        .split(",")
        .map(|s| s.trim())
        .map(|s| s.parse::<i64>().unwrap())
        .collect(),
    );
  }
  let mut sum: i64 = 0;
  let mut previous: Option<i64> = None;
  let mut position: i64 = 0;
  let mut max: i64 = 0;
  let crabs = crabs_opt.unwrap();

  for crab in &crabs {
    if crab > &max {
      max = *crab;
    }
  }
  loop {
    sum = (&crabs).iter().map(|crab| (crab - position).abs()).sum();
    if let Some(old_sum) = previous {
      if old_sum < sum {
        sum = old_sum;
        position -= 1;
        break;
      }
    }
    if position > max {
      panic!("no minimum found")
    }
    previous = Some(sum);
    position += 1
  }
  format!("pos: {}, sum: {}", position, sum)
}
