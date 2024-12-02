
const FILE_PATH: &str = "./day02.txt";
const MAX_SAFE_LEVEL_CHANGE: u32 = 3;

fn are_valid_levels(levels: &Vec<u32>) -> bool {
    for i in 0..levels.len() - 1 {
        if levels[i + 1] <= levels[i] 
            || levels[i + 1] - levels[i] > MAX_SAFE_LEVEL_CHANGE {
            return false;
        }
    }

    true
}

fn main() {
    let mut n_safe_reports: u32 = 0;

    // iterate over lines
    for line in std::fs::read_to_string(FILE_PATH).unwrap().lines() {
        let mut levels: Vec<u32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();

        if vec![levels[0] > levels[1], levels[1] > levels[2], levels[2] > levels[3]].into_iter().filter(|&x| x == true).count() >= 2 { // reverse if in descending order
            levels = levels.into_iter().rev().collect();
        }

        if are_valid_levels(&levels) {
            n_safe_reports += 1;
        } else {
            let levels_len = levels.len();
            for i in 0..levels_len {
                let mut copy = levels.clone();
                copy.remove(i);
                if are_valid_levels(&copy) {
                    n_safe_reports += 1;
                    break;
                }
            }
        }
    }

    println!("{}", n_safe_reports);
}

