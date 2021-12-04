#[derive(Debug, Copy, Clone)]
struct Card([[i32; 5]; 5]);

#[derive(Debug, Clone)]
struct CardPart(Vec<[i32; 5]>);

type Line = [i32; 5];

fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
  v.try_into()
      .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

pub fn main(input_file: String) -> String {
  // let mut numbers: Vec<i32> = vec!();
  let mut bingo_cards: Vec<Card> = vec!();
  let mut current_card: Option<Vec<Line>> = None;
  for (i, line) in crate::read::get_reader(&input_file).enumerate() {
    let line = line.expect("Could not read line");
    println!("line: '{}'", line);
    if i == 0 {
      // numbers = line.split(",").map(|s| s.trim()).map(|s| s.parse().unwrap()).collect();
    } else if line == "" {
      if let Some(card) = current_card {
        let bingo_card: Card = Card(vec_to_array(card));
        bingo_cards.push(bingo_card);
      }
      current_card = Some(vec!());
    } else {
      let line: Line = vec_to_array(line.split(" ").map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.parse().unwrap()).collect());
      let mut card = current_card.unwrap();
      card.push(line);
      current_card = Some(card)
    }
  }

  let answer = 0;
  format!("answer : {}", answer)
}
