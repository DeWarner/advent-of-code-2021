const DAYS: usize = 80;

struct School(Vec<usize>);

impl School {
  fn age(&mut self) {
    let mut addfish: usize = 0;
    for fish in &mut self.0 {
      match fish {
        0 => {
          *fish = 6;
          addfish += 1;
        }
        _ => *fish -= 1,
      }
    }
    for _ in 0..addfish {
      self.0.push(8)
    }
  }
}

pub fn main(input_file: String) -> String {
  let mut school: School = School(vec![]);
  for (i, line) in crate::read::get_reader(&input_file).enumerate() {
    let line = line.expect("Could not read line");
    if i == 0 {
      school = School(
        line
          .split(",")
          .map(|s| s.trim())
          .map(|s| s.parse::<usize>().unwrap())
          .collect(),
      );
    }
  }
  for day in 1..=DAYS {
    school.age();
    println!("{} days: {:?}", day, school.0);
  }
  let ans = school.0.len();
  format!("{}", ans)
}
