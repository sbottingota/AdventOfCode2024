use std::collections::HashMap;

const FILE_PATH: &str = "day11.txt";

struct Stones {
    stones: HashMap<u128, usize>,
}

impl Stones {
    fn new() -> Self {
        Stones { stones: HashMap::new() }
    }

    fn add(&mut self, key: u128, value: usize) {
        if stones.contains_key(&key) {
            *self.stones.get_mut(&key).unwrap() += value;
        } else {
            self.stones.insert(key, value);
        }
    }

    fn reserve(&mut self, additional: usize) {
        self.stones.reserve(additional);
    }

    fn get(&self, key: u128) -> &usize {
        self.stones.get(key)
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
        stones.add(stone, 1);
    }

    println!("{:?}", stones);

    for i in 0..25 {
        println!("{}", i);
        let mut new_stones = Stones::new();
        new_stones.reserve(stones.len());

        for stone in stones {
            if stone == 0 {
                new_stones.add(1, stones.get(stone));
            } else if stone.to_string().len() % 2 == 0 {
                let stone_string = stone.to_string();
                new_stones.add(stone_string[..stone_string.len() / 2].parse().unwrap(), stones.get(stone));
                new_stones.push(stone_string[stone_string.len() / 2..].parse().unwrap());
            } else {
                new_stones.push(stone * 2024);
            }
        }
        stones = new_stones;
    }

    println!("{}", stones.len());
    */
}

