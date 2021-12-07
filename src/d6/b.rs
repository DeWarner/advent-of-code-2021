use std::collections::HashMap;

const DAYS: usize = 256;

struct School(HashMap<usize, u128>);

impl School {
  fn new(school_as_vec: Vec<usize>) -> School {
    let mut school = HashMap::new();
    for fish_ttl in school_as_vec {
      let counter = school.entry(fish_ttl).or_insert(0);
      *counter += 1;
    }
    School(school)
  }

  fn get(&self, ttl: usize) -> u128 {
    *self.0.get(&ttl).unwrap_or(&0)
  }
  fn age(&mut self) {
    let mut next_school: HashMap<usize, u128> = HashMap::new();
    *next_school.entry(6).or_insert(0) += self.get(0);
    *next_school.entry(8).or_insert(0) += self.get(0);
    for ttl in 1..=8 {
      *next_school.entry(ttl - 1).or_insert(0) += self.get(ttl);
    }
    self.0 = next_school;
  }

  fn count(&self) -> u128 {
    let mut total = 0;
    for ttl in 0..=8 {
      total += self.0.get(&ttl).unwrap_or(&0);
    }
    total
  }
}

pub fn main(input_file: String) -> String {
  let mut school_opt: Option<School> = None;
  for (i, line) in crate::read::get_reader(&input_file).enumerate() {
    let line = line.expect("Could not read line");
    if i == 0 {
      school_opt = Some(School::new(
        line
          .split(",")
          .map(|s| s.trim())
          .map(|s| s.parse::<usize>().unwrap())
          .collect(),
      ));
    }
  }
  let mut school = school_opt.unwrap();
  for day in 1..=DAYS {
    school.age();
    println!("{} days: {:?}", day, school.0);
  }
  let ans = school.count();
  format!("{}", ans)
}
