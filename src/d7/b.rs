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
  let mut position: i64 = 0;
  let mut max: i64 = 0;
  let mut ans_position: i64 = 0;
  let mut ans_sum: Option<i64> = None;

  let crabs = crabs_opt.unwrap();
  for crab in &crabs {
    if crab > &max {
      max = *crab;
    }
  }
  loop {
    sum = (&crabs)
      .iter()
      .map(|crab| {
        // println!(
        //   "crab position: {}, move to: {}, distance: {} ",
        //   crab,
        //   position,
        //   (crab - position).abs()
        // );
        (crab - position).abs()
      })
      .map(|movement| {
        // println!("fuel: {}", (movement + 1i64) * movement / 2);
        (movement + 1i64) * movement / 2
      })
      .sum();
    // println!("sum: {}", sum);
    if let Some(old_sum) = ans_sum {
      if old_sum > sum {
        ans_sum = Some(sum);
        ans_position = position;
      }
    } else {
      ans_sum = Some(sum);
      ans_position = position;
    }
    if position > max {
      break;
    }
    position += 1
  }
  let ans = ans_sum.unwrap();
  format!("pos: {}, sum: {}", ans_position, ans)
}
