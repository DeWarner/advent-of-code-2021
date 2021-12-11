use std::collections::HashMap;

// enum Marbles {
//     None,
//     Some(u32),
//     Redirect(usize, usize),
// }

pub fn main(input_file: String) -> String {
    let mut map: Vec<Vec<u32>> = vec![];
    let mut weight_map: HashMap<(usize, usize), u32> = HashMap::new();
    for (i, line) in crate::read::get_reader(&input_file).enumerate() {
        let line = line.expect("Could not read line");
        let row: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        for (j, height) in row.iter().enumerate() {
            if *height != 9u32 {
                weight_map.insert((i, j), 1);
            }
        }
        map.push(row);
    }
    loop {
        let mut changed = false;
        for (i, row) in map.iter().enumerate() {
            for (j, height) in row.iter().enumerate() {
                if let Some(receiver) = lowest_neighbor(&map, i, j, *height) {
                    let passer = *weight_map.get(&(i, j)).unwrap_or(&0);
                    if passer != 0 {
                        println!("moved {},{} to {},{}", i, j, receiver.0, receiver.1);
                        changed = true;
                        *weight_map.entry((i, j)).or_insert(0) = 0;
                        *weight_map.entry(receiver).or_insert(0) += passer;
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }
    let mut values: Vec<u32> = weight_map.values().map(|s| *s).collect();
    values.sort();
    values.reverse();
    for value in &values {
        println!("{}", value);
    }
    println!("top3 {:?}, {:?}, {:?}", values[0], values[1], values[2]);
    let sum = values[0] * values[1] * values[2];
    format!("{}", sum)
}

fn usize_sub(a: usize, sub: i32) -> usize {
    ((a as i32) - sub) as usize
}

fn lowest_neighbor(map: &Vec<Vec<u32>>, i: usize, j: usize, height: u32) -> Option<(usize, usize)> {
    let min = ((i, j), height);
    if let Some(min) = get_min(map, usize_sub(i, 1), j, min) {
        return Some(min.0);
    }
    if let Some(min) = get_min(map, i + 1, j, min) {
        return Some(min.0);
    }
    if let Some(min) = get_min(map, i, usize_sub(j, 1), min) {
        return Some(min.0);
    }
    if let Some(min) = get_min(map, i, j + 1, min) {
        return Some(min.0);
    }
    None
}

fn get_min(
    map: &Vec<Vec<u32>>,
    i: usize,
    j: usize,
    min: ((usize, usize), u32),
) -> Option<((usize, usize), u32)> {
    if let Some(row) = map.get(i) {
        if let Some(height) = row.get(j) {
            if *height < min.1 {
                return Some(((i, j), *height));
            }
        }
    }
    return None;
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
