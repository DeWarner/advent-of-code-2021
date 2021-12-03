pub fn main(input: std::io::Lines<std::io::BufReader<std::fs::File>>) -> String {
  // let mut counter: i32 = 0;
  let mut bit_weights: Option<Vec<i32>> = None;
  for line in input {
    let line = line.expect("Could not read line");
    let characters: Vec<char> = line.chars().collect();
    match &mut bit_weights {
      None => {
        let mut bit_weights_vec = Vec::new();
        for c in characters {
          bit_weights_vec.push(match c {
            '1' => 1,
            '0' => -1,
            other => panic!("invalid binary char {}", other),
          })
        }
        bit_weights = Some(bit_weights_vec);
      }
      Some(weights) => {
        for (i, c) in characters.iter().enumerate() {
          weights[i] += match c {
            '1' => 1,
            '0' => -1,
            other => panic!("invalid binary char {}", other),
          }
        }
      }
    }
  }
  let mut gamma = 0;
  let mut epsilon = 0;
  if let Some(weights) = &bit_weights {
    for (i, w) in weights.iter().enumerate() {
      let power: u32 = (weights.len() - 1 - i).try_into().unwrap();
      if w >= &0i32 {
        gamma += 1 * 2i32.pow(power);
      } else {
        epsilon += 1 * 2i32.pow(power);
      }
    }
  };
  format!(
    "weights: {:?}, gamma: {}, epsilon: {}, answer : {}",
    bit_weights,
    gamma,
    epsilon,
    gamma * epsilon
  )
}
