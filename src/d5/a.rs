struct Map {
  grid: std::collections::HashMap<(usize, usize), u32>,
  max_x: usize,
  max_y: usize,
}

impl Map {
  fn new() -> Map {
    Map {
      grid: std::collections::HashMap::new(),
      max_x: 0,
      max_y: 0,
    }
  }
  fn sum_over_limit(&self, min: u32) -> u32 {
    let mut sum = 0u32;
    for i in 0..=self.max_x {
      for j in 0..=self.max_y {
        if self.grid.get(&(i, j)).unwrap_or(&0u32) >= &min {
          sum += 1;
        }
      }
    }
    return sum;
  }
  fn mark(&mut self, x: usize, y: usize) {
    println!("marking {},{}", x, y);
    let counter = self.grid.entry((x, y)).or_insert(0);

    *counter += 1;
    if self.max_x < x {
      self.max_x = x;
    }
    if self.max_y < y {
      self.max_y = y;
    }
  }
  fn mark_line(&mut self, (ax, ay): (usize, usize), (bx, by): (usize, usize)) {
    if ax == bx {
      let positions = match ay < by {
        true => ay..=by,
        false => by..=ay,
      };
      for y in positions {
        println!("marking vertical line");
        self.mark(ax, y);
      }
    } else if ay == by {
      let positions = match ax < bx {
        true => ax..=bx,
        false => bx..=ax,
      };
      println!("marking horizontal line");
      for x in positions {
        self.mark(x, ay);
      }
    } else {
      println!("skipping diagonal line");
    }
  }
  fn square_fmt(count: u32) -> char {
    match count {
      0 => '.',
      1 => '1',
      _ => '+',
    }
  }
}
impl std::fmt::Display for Map {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let mut map = String::new();
    for y in 0..(self.max_y + 1) {
      for x in 0..(self.max_x + 1) {
        map.push(Map::square_fmt(*self.grid.get(&(x, y)).unwrap_or(&0u32)));
      }
      map.push('\n');
    }
    map.push('\n');
    write!(f, "{}", map)
  }
}

pub fn main(input_file: String) -> String {
  let mut map: Map = Map::new();
  for line in crate::read::get_reader(&input_file) {
    let line = line.expect("Could not read line");
    println!("line:; '{}'", line);
    let mut num_str: String = String::new();
    let mut points: Vec<String> = vec![];
    for c in line.chars() {
      match c {
        '0'..='9' => num_str.push(c),
        ' ' | '-' | '>' | ',' => {
          if num_str != "" {
            points.push(num_str);
            num_str = String::new();
          }
        }
        _ => panic!("unexpected char"),
      }
    }
    if points.len() != 3 {
      panic!("incorrect number of tokens")
    }
    let ax: usize = points[0].parse().unwrap();
    let ay: usize = points[1].parse().unwrap();
    let bx: usize = points[2].parse().unwrap();
    let by: usize = num_str.parse().unwrap();
    println!("marking: {},{} -> {},{}", ax, ay, bx, by);
    map.mark_line((ax, ay), (bx, by));
  }

  println!("{:?}", map.grid);
  println!("{}", map);

  format!("{}", map.sum_over_limit(2))
}
