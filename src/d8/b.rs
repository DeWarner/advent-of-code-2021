pub fn main(input_file: String) -> String {
  let mut sum: i64 = 0;
  for line in crate::read::get_reader(&input_file) {
    let line = line.expect("Could not read line");
    let sides: Vec<&str> = line.split(" | ").map(|e| e.trim()).collect();
    let input_values: Vec<&str> = sides[0].split(" ").map(|e| e.trim()).collect();
    let output_values: Vec<&str> = sides[1].split(" ").map(|e| e.trim()).collect();
    let mut exact1 = "";
    let mut exact2 = "";
    let mut exact3 = "";
    let mut exact4 = "";
    let mut exact5 = "";
    let mut exact6 = "";
    let mut exact7 = "";
    let mut exact8 = "";
    let mut exact9 = "";
    let mut exact0 = "";
    let mut contains5: Vec<&str> = vec![];
    let mut contains6: Vec<&str> = vec![];
    for value in input_values {
      match value.len() {
        2 => {
          // is 1
          exact1 = value;
        }
        3 => {
          // is 7
          exact7 = value;
        }
        4 => {
          // is 4
          exact4 = value;
        }
        7 => {
          // is 8
          exact8 = value;
        }
        6 => {
          // is either 6, 0, 9
          // 9 contains 4
          // 0 contains 7
          contains6.push(value)
        }
        5 => {
          // is either 2, 5, 3
          // 3 contains 7
          // 2 contains what is in 1 that is not in 6
          contains5.push(value)
        }
        _ => panic!("invalid charset"),
      }
    }
    'propositions: for (i, value) in contains6.iter().enumerate() {
      for c in exact4.chars() {
        if !value.contains(c) {
          continue 'propositions;
        }
      }
      exact9 = value;
      contains6.remove(i);
      break;
    }

    'propositions: for (i, value) in contains6.iter().enumerate() {
      for c in exact7.chars() {
        if !value.contains(c) {
          continue 'propositions;
        }
      }
      exact0 = value;
      contains6.remove(i);
      break;
    }
    exact6 = contains6[0];

    // now for contains5 unpacking
    'propositions: for (i, value) in contains5.iter().enumerate() {
      for c in exact7.chars() {
        if !value.contains(c) {
          continue 'propositions;
        }
      }
      exact3 = value;
      contains5.remove(i);
      break;
    }

    let twos_unique_char = match exact6.contains(exact1.chars().nth(0).unwrap()) {
      true => exact1.chars().nth(1).unwrap(),
      false => exact1.chars().nth(0).unwrap(),
    };
    'propositions: for (i, value) in contains5.iter().enumerate() {
      if !value.contains(twos_unique_char) {
        continue 'propositions;
      }
      exact2 = value;
      contains5.remove(i);
      break;
    }
    exact5 = contains5[0];

    println!("{}", exact0);
    println!("{}", exact1);
    println!("{}", exact2);
    println!("{}", exact3);
    println!("{}", exact4);
    println!("{}", exact5);
    println!("{}", exact6);
    println!("{}", exact7);
    println!("{}", exact8);
    println!("{}", exact9);

    let mut output = 0;
    for value in output_values {
      let digit = if sameish(value, exact0) {
        0
      } else if sameish(value, exact1) {
        1
      } else if sameish(value, exact2) {
        2
      } else if sameish(value, exact3) {
        3
      } else if sameish(value, exact4) {
        4
      } else if sameish(value, exact5) {
        5
      } else if sameish(value, exact6) {
        6
      } else if sameish(value, exact7) {
        7
      } else if sameish(value, exact8) {
        8
      } else if sameish(value, exact9) {
        9
      } else {
        panic!("pattern not found: '{}'", value)
      };
      output = 10 * output + digit;
    }
    println!("output: {}", output);
    sum += output;
  }
  format!("{}", sum)
}

fn sameish(a: &str, b: &str) -> bool {
  for c in a.chars() {
    if !b.contains(c) {
      return false;
    }
  }
  for c in b.chars() {
    if !a.contains(c) {
      return false;
    }
  }
  return true;
}
