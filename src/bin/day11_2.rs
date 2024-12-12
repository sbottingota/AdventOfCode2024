use std::collections::HashMap;

const FILE_PATH: &str = "day11.txt";

type Stones = HashMap<u128, usize>;

fn add(stones: &mut Stones, key: u128, value: usize) {
    if stones.contains_key(&key) {
        *stones.get_mut(&key).unwrap() += value;
    } else {
        stones.insert(key, value);
    }
}

fn main() {
    let stones_vec: Vec<u128> = std::fs::read_to_string(FILE_PATH)
        .expect("Error reading input file")
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut stones = Stones::new();
    for stone in stones_vec {
        add(&mut stones, stone, 1);
    }

    for _ in 0..75 {
        let mut new_stones = Stones::new();
        new_stones.reserve(stones.len());

        for (stone, count) in stones {
            if stone == 0 {
                add(&mut new_stones, 1, count);
            } else if stone.to_string().len() % 2 == 0 {
                let stone_string = stone.to_string();
                add(&mut new_stones, stone_string[..stone_string.len() / 2].parse().unwrap(), count);
                add(&mut new_stones, stone_string[stone_string.len() / 2..].parse().unwrap(), count);
            } else {
                add(&mut new_stones, stone * 2024, count);
            }
        }
        stones = new_stones;
    }

    let mut stone_count = 0_usize;

    for count in stones.values() {
        stone_count += count;
    }

    println!("{}", stone_count);
}

