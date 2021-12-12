pub fn main(input_file: String) -> String {
  let mut scores: Vec<u128> = vec![];
  for line in crate::read::get_reader(&input_file) {
    let line = line.expect("Could not read line");
    if let Some(score) = parse(line) {
      scores.push(score)
    }
  }
  scores.sort();
  let score_index = (scores.len() - 1) / 2;
  println!("score: {:?}", scores);
  println!("score: {}", score_index);
  format!("{}", scores[score_index])
}

fn untangle(line: String) -> String {
  let mut new_line = line.clone();
  for index in (1..new_line.chars().count()).rev() {
    let next_char = new_line.chars().nth(index).unwrap();
    let paired = match new_line.chars().nth(index - 1).unwrap() {
      '{' => {
        if next_char == '}' {
          true
        } else {
          false
        }
      }
      '(' => {
        if next_char == ')' {
          true
        } else {
          false
        }
      }
      '<' => {
        if next_char == '>' {
          true
        } else {
          false
        }
      }
      '[' => {
        if next_char == ']' {
          true
        } else {
          false
        }
      }
      _ => false,
    };
    if paired {
      new_line.remove(index);
      new_line.remove(index - 1);
      break;
    }
  }
  // println!("{}    to    {}", line, new_line);
  if new_line.len() < line.len() {
    new_line = untangle(new_line);
  }
  new_line
}

fn parse(line: String) -> Option<u128> {
  let enders = &[')', ']', '}', '>'][..];
  let remaining = untangle(line);
  if remaining.contains(enders) {
    return None;
  }
  println!("remaining: {}", remaining);
  let mut score: u128 = 0;
  for c in remaining.chars().rev() {
    score = match c {
      '(' => score * 5 + 1,
      '[' => score * 5 + 2,
      '{' => score * 5 + 3,
      '<' => score * 5 + 4,
      _ => panic!("bad char: {}", c),
    };
  }
  println!("score: {}", score);
  Some(score)
}
