const FILE_PATH: &str = "day11.txt";

fn main() {
    let mut stones: Vec<u128> = std::fs::read_to_string(FILE_PATH)
        .expect("Error reading input file")
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    for i in 0..25 {
        println!("{}", i);
        let mut new_stones = Vec::new();
        for stone in stones {
            if stone == 0 {
                new_stones.push(1);
            } else if stone.to_string().len() % 2 == 0 {
                let stone_string = stone.to_string();
                new_stones.push(stone_string[..stone_string.len() / 2].parse().unwrap());
                new_stones.push(stone_string[stone_string.len() / 2..].parse().unwrap());
            } else {
                new_stones.push(stone * 2024);
            }
        }
        stones = new_stones;
    }

    println!("{}", stones.len());
}

