struct Card([Line; 5]);

impl Card {
  fn mark(&mut self, number: i32) {
    for line in &mut self.0 {
      for square in &mut line.0.iter_mut() {
        square.mark(number);
      }
    }
  }
  fn check(&self) -> bool {
    'rows: for i in 0..5 {
      for j in 0..5 {
        if let Square::Unmarked(_) = self.0[i].0[j] {
          continue 'rows;
        }
      }
      return true;
    }
    'cols: for j in 0..5 {
      for i in 0..5 {
        if let Square::Unmarked(_) = self.0[i].0[j] {
          continue 'cols;
        }
      }
      return true;
    }
    false
  }
  fn sum_unmarked(&self) -> i32 {
    let mut sum = 0i32;
    for i in 0..5 {
      for j in 0..5 {
        if let Square::Unmarked(num) = self.0[i].0[j] {
          sum += num;
        }
      }
    }
    return sum;
  }
}

impl std::fmt::Display for Card {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "{}\n{}\n{}\n{}\n{}\n",
      self.0[0], self.0[1], self.0[2], self.0[3], self.0[4]
    )
  }
}

enum Square {
  Unmarked(i32),
  Marked(i32),
}
impl Square {
  fn mark(&mut self, called_number: i32) {
    if let Square::Unmarked(number) = *self {
      if number == called_number {
        *self = Square::Marked(number);
      }
    }
  }
}

impl std::fmt::Display for Square {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Square::Marked(num) => write!(f, " [{:>2}] ", num),
      Square::Unmarked(num) => write!(f, " {:>3}  ", num),
    }
  }
}

struct Line([Square; 5]);

impl std::fmt::Display for Line {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "{}{}{}{}{}",
      self.0[0], self.0[1], self.0[2], self.0[3], self.0[4]
    )
  }
}

fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
  v.try_into()
    .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

pub fn main(input_file: String) -> String {
  let mut numbers: Vec<i32> = vec![];
  let mut bingo_cards: Vec<Card> = vec![];
  let mut current_card: Option<Vec<Line>> = None;
  for (i, line) in crate::read::get_reader(&input_file).enumerate() {
    let line = line.expect("Could not read line");
    // println!("line: '{}'", line);
    if i == 0 {
      numbers = line
        .split(",")
        .map(|s| s.trim())
        .map(|s| s.parse().unwrap())
        .collect();
    } else if line == "" {
      if let Some(card) = current_card {
        let bingo_card: Card = Card(vec_to_array(card));
        bingo_cards.push(bingo_card);
      }
      current_card = Some(vec![]);
    } else {
      let line = Line(vec_to_array(
        line
          .split(" ")
          .map(|s| s.trim())
          .filter(|s| !s.is_empty())
          .map(|s| Square::Unmarked(s.parse().unwrap()))
          .collect(),
      ));
      let mut card = current_card.unwrap();
      card.push(line);
      current_card = Some(card)
    }
  }
  let mut unmarked: i32 = 0;
  let mut last_num: i32 = 0;
  let mut score: i32 = 0;
  let mut remove: Option<Vec<usize>> = None;
  for number in numbers {
    for (i, card) in &mut bingo_cards.iter_mut().enumerate() {
      card.mark(number);
      println!("{}", card);
      if card.check() {
        unmarked = card.sum_unmarked();
        last_num = number;
        score = number * unmarked;
        // break 'caller
        match &mut remove {
          Some(v) => v.push(i),
          None => remove = Some(vec![i]),
        };
      }
    }
    if let Some(v) = &remove {
      println!("removing: {:?}", v);
      for i in v.iter().rev() {
        bingo_cards.remove(*i);
      }
    }
    remove = None;
    println!("");
  }

  format!(
    "num: {:?}, sum: {:?}, answer : {:?}",
    last_num, unmarked, score
  )
}
