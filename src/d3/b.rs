#[derive(Debug)]
enum Round {
  None,
  Exact(String, i32),
  NextWeight(String, i32),
}
fn flag_to_weight(c: char) -> i32 {
  match c {
    '1' => 1,
    '0' => -1,
    other => panic!("invalid binary char {}", other),
  }
}
fn weight_to_flag(w: i32, not: bool) -> char {
  let condition = match not {
    true => w < 0,
    false => w >= 0,
  };
  match condition {
    false => '0',
    true => '1',
  }
}

fn searcher(input_file: &str, pattern: String, not: bool) -> String {
  let mut round: Round = Round::None;

  for line in crate::read::get_reader(&input_file) {
    let line = line.expect("Could not read line");

    if line.starts_with(&pattern) {
      let characters: Vec<char> = line.chars().collect();
      let next_weight = flag_to_weight(characters[pattern.len()]);
      round = match round {
        Round::None => Round::Exact(line.to_string(), next_weight),
        Round::Exact(line, weight) => Round::NextWeight(line, weight + next_weight),
        Round::NextWeight(line, weight) => Round::NextWeight(line, weight + next_weight),
      };
      println!("line {} , pattern {}, round {:?}", &line, pattern, round);
    }
  }
  println!("pattern: {}, round: {:?}", pattern, round);
  match round {
    Round::None => panic!("no matches found, error!"),
    Round::Exact(line, _) => return line,
    Round::NextWeight(line, weight) => {
      if pattern.len() == line.len() - 1 {
        return format!("{}{}", pattern, weight_to_flag(weight, not));
      }
      return searcher(
        input_file,
        format!("{}{}", pattern, weight_to_flag(weight, not)),
        not,
      );
    }
  }
}

pub fn main(input_file: String) -> String {
  // let mut counter: i32 = 0;
  let mut first_weight: i32 = 0;

  for line in crate::read::get_reader(&input_file) {
    let line = line.expect("Could not read line");
    let characters: Vec<char> = line.chars().collect();
    first_weight += flag_to_weight(characters[0]);
  }

  let o2_first_bit = weight_to_flag(first_weight, false);
  let o2_pattern: String = o2_first_bit.to_string();
  let o2_str = searcher(&input_file, o2_pattern, false);
  println!("o2str {}", o2_str);

  let co2_first_bit = weight_to_flag(first_weight, true);
  let co2_pattern: String = co2_first_bit.to_string();
  let co2_str = searcher(&input_file, co2_pattern, true);
  println!("co2str {}", co2_str);

  let o2 = isize::from_str_radix(&o2_str, 2).unwrap();
  let co2 = isize::from_str_radix(&co2_str, 2).unwrap();
  format!("o2: {}, co2: {}, answer : {}", o2, co2, o2 * co2)
}
