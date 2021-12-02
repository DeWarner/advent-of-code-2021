struct Window(i32, i32, i32);
struct Frame {
  window: Window,
  full: bool,
  pos: usize,
  sum: i32,
}

impl Frame {
  fn sum(&mut self) -> i32 {
    if self.full {
      self.sum = self.window.0 + self.window.1 + self.window.2;
      return self.sum;
    } else {
      return 0;
    }
  }

  fn push(&mut self, value: i32) -> Option<i32> {
    match self.pos {
      0 => {
        self.window.0 = value;
      }
      1 => {
        self.window.1 = value;
      }
      2 => {
        self.window.2 = value;
      }
      _ => panic!("invalid pos: {}", self.pos),
    }
    let res = match self.full {
      true => {
        let prev_sum = self.sum;
        let new_sum = self.sum();
        Some(new_sum - prev_sum)
      }
      false => None,
    };

    if !self.full && self.pos == 2 {
      self.full = true;
    }
    println!(
      "pos = {}, pos + 1 = {}, pos+1%3 = {}",
      self.pos,
      self.pos + 1,
      (self.pos + 1) % 3
    );
    self.pos = (self.pos + 1) % 3;
    res
  }
}

pub fn main(input: std::io::Lines<std::io::BufReader<std::fs::File>>) -> String {
  let mut frame = Frame {
    window: Window(0, 0, 0),
    full: false,
    pos: 0,
    sum: 0,
  };
  let mut increases = 0;

  for line in input {
    let line = line.expect("Could not read line from standard in");
    let new_depth = match line.parse::<i32>() {
      Ok(num) => num,
      Err(err) => panic!(
        "Invalid depth, could not parse into a positive integer '{}', {:?}",
        line, err
      ),
    };
    if let Some(change) = frame.push(new_depth) {
      if change > 0 {
        increases += 1;
      }
    }
  }
  format!("{}", increases)
}
