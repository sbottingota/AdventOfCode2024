use std::cmp::{min, max};

const FILE_PATH: &str = "./day01.txt";

fn main() {
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    for line in std::fs::read_to_string(FILE_PATH).unwrap().lines() {
        let mut line_iter = line.split_whitespace();
        left_list.push(line_iter.next().unwrap().parse().unwrap());
        right_list.push(line_iter.next().unwrap().parse().unwrap());
    }

    left_list.sort();
    right_list.sort();

    let mut differences_sum: u32 = 0;

    for (left, right) in left_list.into_iter().zip(right_list) {
        differences_sum += max(left, right) - min(left, right);
    }

    println!("{}", differences_sum);
}

