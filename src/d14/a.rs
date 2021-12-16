use std::collections::HashMap;

pub fn main(input_file: String) -> String {
  let mut chain: Vec<char> = Vec::new();
  let mut reactions: HashMap<(char, char), char> = HashMap::new();
  let mut folding = false;
  for line in crate::read::get_reader(&input_file) {
    let line = line.expect("Could not read line");
    if line == "" {
      folding = true;
      continue;
    }
    if !folding {
      chain = line.chars().collect();
    } else {
      let a = line.chars().nth(0).unwrap();
      let b = line.chars().nth(1).unwrap();
      let product = line.chars().nth(6).unwrap();
      reactions.insert((a, b), product);
    }
  }
  println!("{}", chain.iter().cloned().collect::<String>());
  for _ in 0..10 {
    chain = next(chain, &reactions);
    println!("{}", chain.iter().cloned().collect::<String>());
  }

  let mut charmap: HashMap<char, u32> = HashMap::new();
  for c in chain {
    let count = charmap.entry(c).or_insert(0);
    *count += 1;
  }
  let mut max = (' ', 0);
  let mut min = (' ', 10000000);
  for (key, value) in charmap {
    println!("{} / {}", key, value);
    if value < min.1 {
      min = (key, value)
    }
    if value > max.1 {
      max = (key, value)
    }
  }

  format!("{}", max.1 - min.1)
}

fn next(chain: Vec<char>, reactions: &HashMap<(char, char), char>) -> Vec<char> {
  let mut new_chain: Vec<char> = vec![chain[0]];
  for i in 1..chain.len() {
    let a = chain[i - 1];
    let b = chain[i];
    let result = match reactions.get(&(a, b)) {
      Some(x) => vec![*x, b],
      None => vec![b],
    };
    // println!("({}{}) result: {}", a, b, result);
    new_chain.extend(result);
  }
  new_chain
}
