pub fn main(input_file: String) -> String {
  let mut map: Vec<Vec<u32>> = vec![];
  let mut sum: u32 = 0;
  for (i, line) in crate::read::get_reader(&input_file).enumerate() {
    let line = line.expect("Could not read line");
    let row: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
    map.push(row);
  }
  for (i, row) in map.iter().enumerate() {
    for (j, height) in row.iter().enumerate() {
      if is_lower_than(&map, usize_sub(i, 1), j, *height) {
        continue;
      }
      if is_lower_than(&map, i + 1, j, *height) {
        continue;
      }
      if is_lower_than(&map, i, usize_sub(j, 1), *height) {
        continue;
      }
      if is_lower_than(&map, i, j + 1, *height) {
        continue;
      }
      println!("found min ({},{}) => {}", i, j, height);
      sum += height + 1;
    }
  }
  format!("{}", sum)
}

fn usize_sub(a: usize, sub: i32) -> usize {
  ((a as i32) - sub) as usize
}

fn is_lower_than(map: &Vec<Vec<u32>>, i: usize, j: usize, target: u32) -> bool {
  if let Some(row) = map.get(i) {
    if let Some(height) = row.get(j) {
      if target >= *height {
        return true;
      }
    }
  }
  println!("point ({},{}) is bigger than => {}", i, j, target);
  false
}
