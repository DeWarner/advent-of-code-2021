pub fn main(input_file: String) -> String {
  let mut count: i64 = 0;
  for line in crate::read::get_reader(&input_file) {
    let line = line.expect("Could not read line");
    let sides: Vec<&str> = line.split(" | ").map(|e| e.trim()).collect();
    let output_values: Vec<&str> = sides[1].split(" ").map(|e| e.trim()).collect();
    for value in output_values {
      match value.len() {
        2 | 3 | 4 | 7 => count += 1,
        _ => continue,
      }
    }
    println!("line: {}, new count: {}", line, count);
  }
  format!("{}", count)
}
