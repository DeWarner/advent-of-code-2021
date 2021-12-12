pub fn main(input_file: String) -> String {
  let mut sum: u32 = 0;
  for line in crate::read::get_reader(&input_file) {
    let line = line.expect("Could not read line");
    sum += parse(line);
  }
  format!("{}", sum)
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

fn parse(line: String) -> u32 {
  let mut remaining = untangle(line);
  remaining.retain(|c| !"([{<".contains(c));
  println!("remaining: {}", remaining);
  let first_bad = remaining.chars().nth(0);
  match first_bad {
    Some(')') => 3,
    Some(']') => 57,
    Some('}') => 1197,
    Some('>') => 25137,
    Some(c) => panic!("unexpected char {}", c),
    None => 0,
  }
}
