const FILE_PATH: &str = "./day02.txt";
const MAX_SAFE_LEVEL_CHANGE: u32 = 3;

fn main() {
    let mut n_safe_reports: u32 = 0;

    // iterate over lines
    'outer: for line in std::fs::read_to_string(FILE_PATH).unwrap().lines() {
        let mut levels: Vec<u32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();

        if levels[0] > levels[1] { // reverse if in descending order
            levels = levels.into_iter().rev().collect();
        }

        for i in 0..levels.len() - 1 {
            if levels[i + 1] <= levels[i] 
                || levels[i + 1] - levels[i] > MAX_SAFE_LEVEL_CHANGE {
                continue 'outer;
            }
        }

        n_safe_reports += 1;
    }

    println!("{}", n_safe_reports);
}

