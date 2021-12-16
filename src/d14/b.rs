use std::collections::HashMap;

pub fn main(input_file: String) -> String {
    let mut chain: HashMap<(char, char), u128> = HashMap::new();
    let mut reactions: HashMap<(char, char), char> = HashMap::new();
    let mut leader: char = '.';
    let mut folding = false;

    for line in crate::read::get_reader(&input_file) {
        let line = line.expect("Could not read line");
        if line == "" {
            folding = true;
            continue;
        }
        if !folding {
            let line: Vec<char> = line.chars().collect();
            for i in 1..line.len() {
                let pair_count = chain.entry((line[i - 1], line[i])).or_insert(0);
                *pair_count += 1;
            }
            leader = line[0];
        } else {
            let a = line.chars().nth(0).unwrap();
            let b = line.chars().nth(1).unwrap();
            let product = line.chars().nth(6).unwrap();
            reactions.insert((a, b), product);
        }
    }
    println!("{:?}", chain);
    for i in 0..40 {
        chain = next(&mut chain, &reactions);
        println!("{}", i);
        println!("{:?}", chain);
    }

    let mut charmap: HashMap<char, u128> = HashMap::new();
    for ((_a, b), count) in chain.iter() {
        let b_count = charmap.entry(*b).or_insert(0);
        *b_count += count;
    }
    *charmap.entry(leader).or_insert(0) += 1u128;

    let mut max: (char, u128) = (' ', 0);
    let mut min: (char, u128) = (' ', 1000000000000000000);
    for (key, value) in charmap {
        println!("{} / {}", key, value);
        if value < min.1 {
            min = (key, value)
        }
        if value > max.1 {
            max = (key, value)
        }
    }

    format!("{}", max.1 - min.1)
}

fn next(
    chain: &mut HashMap<(char, char), u128>,
    reactions: &HashMap<(char, char), char>,
) -> HashMap<(char, char), u128> {
    let mut new_chain: HashMap<(char, char), u128> = HashMap::new();
    for (key, value) in chain.iter_mut() {
        if let Some(x) = reactions.get(key) {
            *(new_chain.entry((key.0, *x)).or_insert(0)) += *value;
            *(new_chain.entry((*x, key.1)).or_insert(0)) += *value;
        }
    }
    new_chain
}
