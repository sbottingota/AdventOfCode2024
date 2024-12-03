const FILE_PATH: &str = "./day01.txt";

fn main() {
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    for line in std::fs::read_to_string(FILE_PATH).unwrap().lines() {
        let mut line_iter = line.split_whitespace();
        left_list.push(line_iter.next().unwrap().parse().unwrap());
        right_list.push(line_iter.next().unwrap().parse().unwrap());
    }

    let mut similarity_score: u32 = 0;

    for n in left_list {
        similarity_score += n * right_list.iter().filter(|&&m| n == m).count() as u32;
    }

    println!("{}", similarity_score);
}

