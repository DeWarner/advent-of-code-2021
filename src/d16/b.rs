use std::collections::HashSet;

struct Grid {
  grid: HashSet<(usize, usize)>,
  max_x: usize,
  max_y: usize,
}
impl Grid {
  fn new() -> Grid {
    Grid {
      grid: HashSet::new(),
      max_x: 0,
      max_y: 0,
    }
  }
  fn add_line(&mut self, line: &str) {
    let coord: Vec<usize> = line.split(",").map(|s| s.parse().unwrap()).collect();
    let x = coord.get(0).unwrap();
    let y = coord.get(1).unwrap();
    self.add(*x, *y);
  }
  fn add(&mut self, x: usize, y: usize) {
    self.grid.insert((x, y));
    if self.max_x < x {
      self.max_x = x;
    }
    if self.max_y < y {
      self.max_y = y;
    }
  }
  fn count(&self) -> usize {
    self.grid.len()
  }
  fn fold(self, fold: &Fold) -> Grid {
    let mut grid = Grid::new();
    match fold {
      Fold::X(x) => {
        let x = *x;
        for coord in &self.grid {
          let i = coord.0;
          let j = coord.1;
          if i == x {
            continue;
          } else if i > x {
            grid.add(2 * x - i, j);
          } else {
            grid.add(i, j);
          }
        }
      }
      Fold::Y(y) => {
        let y = *y;
        for coord in &self.grid {
          let i = coord.0;
          let j = coord.1;

          if j == y {
            continue;
          } else if j > y {
            grid.add(i, 2 * y - j);
          } else {
            grid.add(i, j);
          }
        }
      }
    };
    grid
  }
}
impl std::fmt::Display for Grid {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let mut map = String::new();
    for y in 0..=self.max_y {
      for x in 0..=self.max_x {
        let rep = match self.grid.contains(&(x, y)) {
          false => '.',
          true => '#',
        };
        map.push(rep)
      }
      map.push('\n');
    }
    map.push('\n');
    write!(f, "{}", map)
  }
}

enum Fold {
  X(usize),
  Y(usize),
}

impl Fold {
  fn new(line: &str) -> Fold {
    let orientation = line.chars().nth(11).unwrap();
    let level = &line[13..];
    let level_usize: usize = level.parse().unwrap();
    match orientation {
      'x' => Fold::X(level_usize),
      'y' => Fold::Y(level_usize),
      _ => panic!("unexpected format: {}", line),
    }
  }
}

pub fn main(input_file: String) -> String {
  let mut grid = Grid::new();
  let mut folds: Vec<Fold> = Vec::new();
  let mut folding = false;
  for line in crate::read::get_reader(&input_file) {
    let line = line.expect("Could not read line");
    if line == "" {
      folding = true;
      continue;
    }
    if !folding {
      grid.add_line(&line);
    } else {
      folds.push(Fold::new(&line));
    }
  }
  println!("{}", grid);
  for fold in folds {
    grid = grid.fold(&fold);
    println!("{}", grid);
  }

  format!("{}", grid.count())
}
