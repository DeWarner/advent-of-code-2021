use std::collections::{HashMap, HashSet};

struct Node {
  paths: HashSet<String>,
}

impl Node {
  fn new() -> Node {
    Node {
      paths: HashSet::new(),
    }
  }

  fn add(&mut self, to: &str) {
    self.paths.insert(to.to_string());
  }
}

struct Nodes {
  nodes: HashMap<String, Node>,
}

impl Nodes {
  fn new() -> Nodes {
    Nodes {
      nodes: HashMap::new(),
    }
  }

  fn add(&mut self, to: &str, from: &str) {
    {
      let a = self.nodes.entry(to.to_string()).or_insert(Node::new());
      a.add(from);
    }
    {
      let b = self.nodes.entry(from.to_string()).or_insert(Node::new());
      b.add(to);
    }
  }
  fn get(&self, name: &str) -> &HashSet<String> {
    &self.nodes.get(name).unwrap().paths
  }

  fn finish_path(&self, before: Vec<String>, end: &str) -> Vec<Vec<String>> {
    let mut paths: Vec<Vec<String>> = vec![];
    for node_name in self.get(before.last().unwrap()) {
      let mut new_path = before.clone();
      new_path.push(node_name.to_string());
      if before.contains(node_name) && node_name.chars().any(|c| c.is_lowercase()) {
        continue;
      }
      if node_name == end {
        paths.push(new_path);
      } else {
        paths.append(&mut self.finish_path(new_path, end));
      }
    }
    paths
  }
}

pub fn main(input_file: String) -> String {
  let mut graph = Nodes::new();
  for line in crate::read::get_reader(&input_file) {
    let line = line.expect("Could not read line");
    let parts: Vec<String> = line.split("-").map(|s| s.to_string()).collect();
    graph.add(parts.get(0).unwrap(), parts.get(1).unwrap());
  }
  let paths = graph.finish_path(vec!["start".to_string()], "end");
  println!("paths: \n{:?}", paths);
  format!("{}", paths.len())
}
