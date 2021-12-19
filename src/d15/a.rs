use std::collections::HashMap;

struct Map {
  grid: Vec<Vec<u32>>,
  best: HashMap<(usize, usize), u32>,
}

impl std::fmt::Display for Map {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let mut map = String::new();
    for row in &self.grid {
      for risk in row {
        map.push_str(format!("{},\t", risk).as_str());
      }
      map.push('\n');
    }
    map.push('\n');

    for i in 0..self.grid.len() {
      for j in 0..self.grid[0].len() {
        match self.best.get(&(i, j)) {
          Some(best) => map.push_str(format!("{},\t", best).as_str()),
          None => map.push('_'),
        }
      }
      map.push('\n');
    }
    map.push('\n');
    map.push_str(format!("{:?}", self.best).as_str());
    write!(f, "{}", map)
  }
}

pub fn main(input_file: String) -> String {
  let mut map: Map = Map {
    grid: vec![],
    best: HashMap::new(),
  };
  map.best.insert((0, 0), 0);
  for line in crate::read::get_reader(&input_file) {
    let line = line.expect("Could not read line");
    let row: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
    map.grid.push(row);
  }
  println!("{}", map);
  let mut round: Vec<((usize, usize), u32)>;
  let mut next_round: Vec<((usize, usize), u32)> = vec![((0, 0), 0)];
  loop {
    if next_round.len() == 0 {
      break;
    }
    round = next_round;
    next_round = Vec::new();
    round.sort_by(|(_, a), (_, b)| a.cmp(b));
    for (tile, _best) in round {
      next_round.append(&mut scout(tile, &mut map));
    }
  }
  println!("{}", map);
  format!(
    "{}",
    map
      .best
      .get(&(map.grid.len() - 1, map.grid[0].len() - 1))
      .unwrap()
  )
}

fn scout(start: (usize, usize), map: &mut Map) -> Vec<((usize, usize), u32)> {
  println!("scouting: {:?}", start);
  let neighbours = get_neighbours(start, map);
  let mut next_round: Vec<((usize, usize), u32)> = Vec::new();
  for neighbour in neighbours {
    let current_risk = *map.best.get(&start).unwrap();
    let neighbour_risk = map.grid[neighbour.0][neighbour.1];
    let new_neighbour_risk = current_risk + neighbour_risk;
    if let Some(neighbour_best) = map.best.get(&neighbour) {
      if *neighbour_best <= new_neighbour_risk {
        continue;
      }
    }
    println!("setting best {:?}, {}", neighbour, new_neighbour_risk);
    map.best.insert(neighbour, new_neighbour_risk);
    next_round.push((neighbour, new_neighbour_risk));
  }
  next_round
}

fn get_neighbours(pos: (usize, usize), map: &Map) -> Vec<(usize, usize)> {
  let mut result = Vec::new();
  let (i, j) = pos;
  if (i as i32 - 1) >= 0 {
    if check_tile_exists(usize_sub(i, 1), j, &map) {
      result.push((usize_sub(i, 1), j));
    }
  }
  if (j as i32 - 1) >= 0 {
    if check_tile_exists(i, usize_sub(j, 1), &map) {
      result.push((i, usize_sub(j, 1)));
      result.push((i, usize_sub(j, 1)));
    }
  }
  if check_tile_exists(i, j + 1, &map) {
    result.push((i, j + 1));
  }
  if check_tile_exists(i + 1, j, &map) {
    result.push((i + 1, j));
  }
  result
}

fn check_tile_exists(i: usize, j: usize, map: &Map) -> bool {
  match map.grid.get(i) {
    Some(row) => match row.get(j) {
      Some(_) => true,
      None => false,
    },
    None => false,
  }
}

fn usize_sub(a: usize, sub: i32) -> usize {
  ((a as i32) - sub) as usize
}
