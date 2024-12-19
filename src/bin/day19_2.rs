use std::collections::HashMap;

const FILE_PATH: &str = "day19.txt";

// match the items of short off from long
fn match_off(long: &[char], short: &[char]) -> Option<Vec<char>> {
    if long.len() < short.len() {
        return None;
    }

    let mut long_mut = long.to_vec();
    for c in short.iter().rev() {
        if long_mut.last().unwrap() == c {
            let _ = long_mut.pop();
        } else {
            return None;
        }
    }

    Some(long_mut)
}

fn count_combinations(pattern: &[char], parts: &[Vec<char>]) -> usize {
    let mut states: HashMap<Vec<char>, usize> = HashMap::from([(pattern.to_vec(), 1_usize)]);

    let mut ret = 0;

    while let Some(next) = states.clone().keys().next() {
        let count = states.remove(next).unwrap();

        for part in parts {
            if let Some(matched_off) = match_off(next, part) {
                if matched_off.is_empty() {
                    ret += count;
                } else if let Some(old) = states.insert(matched_off.clone(), count) {
                    *states.get_mut(&matched_off).unwrap() += old;
                }
            }
        }
    }

    ret
}

fn main() {
    let input = std::fs::read_to_string(FILE_PATH).expect("Error reading input file");


    let mut lines = input.lines();

    let parts: Vec<Vec<char>> = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|substr| substr.chars().collect())
        .collect();

    let _ = lines.next();

    let patterns: Vec<Vec<char>> = lines
        .map(|line| line.chars().collect())
        .collect();

    let mut count = 0_usize;
    for pattern in patterns {
        count += count_combinations(&pattern, &parts);
    }

    println!("{}", count);
}

