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

fn can_form_pattern(pattern: &[char], parts: &[Vec<char>]) -> bool {
    let mut states: Vec<Vec<char>> = vec![pattern.to_vec()];

    while let Some(next) = states.pop() {
        for part in parts {
            if let Some(matched_off) = match_off(&next, part) {
                if matched_off.is_empty() {
                    return true;
                }

                states.push(matched_off);
            }
        }
    }

    false
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
        if can_form_pattern(&pattern, &parts) {
            count += 1;
        }
    }

    println!("{}", count);
}

