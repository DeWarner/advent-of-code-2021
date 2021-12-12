struct Grid(Vec<Vec<u32>>);

impl std::fmt::Display for Grid {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let mut map = String::new();
    for row in &self.0 {
      for octo in row {
        map.push(std::char::from_digit(*octo, 10).unwrap());
      }
      map.push('\n');
    }
    map.push('\n');
    write!(f, "{}", map)
  }
}

pub fn main(input_file: String) -> String {
  let mut map: Grid = Grid(vec![]);
  let mut sum: u32 = 0;
  for line in crate::read::get_reader(&input_file) {
    let line = line.expect("Could not read line");
    let row: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
    map.0.push(row);
  }
  println!("{}", map);
  for _ in 0..100 {
    step(&mut map, &mut sum);
    println!("{}", map);
  }
  format!("{}", sum)
}

fn step(map: &mut Grid, sum: &mut u32) {
  for row in map.0.iter_mut() {
    for energy in row.iter_mut() {
      *energy += 1;
    }
  }
  for i in 0..map.0.len() {
    for j in 0..map.0[0].len() {
      explode(map, sum, i, j);
    }
  }
}

fn explode(map: &mut Grid, sum: &mut u32, i: usize, j: usize) {
  let octopus = map.0.get_mut(i).unwrap().get_mut(j).unwrap();
  if *octopus > 9 {
    *sum += 1;
    *octopus = 0;
    for (x, y) in get_neighbors(i, j) {
      if let Some(row) = map.0.get_mut(x) {
        if let Some(neighbour) = row.get_mut(y) {
          if *neighbour != 0 {
            *neighbour += 1;
            explode(map, sum, x, y);
          }
        }
      }
    }
  }
}

fn get_neighbors(i: usize, j: usize) -> [(usize, usize); 8] {
  [
    (usize_sub(i, 1), usize_sub(j, 1)),
    (usize_sub(i, 1), j),
    (usize_sub(i, 1), j + 1),
    (i, j + 1),
    (i, usize_sub(j, 1)),
    (i + 1, usize_sub(j, 1)),
    (i + 1, j),
    (i + 1, j + 1),
  ]
}

fn usize_sub(a: usize, sub: i32) -> usize {
  ((a as i32) - sub) as usize
}
