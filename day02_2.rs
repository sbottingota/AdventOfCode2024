const FILE_PATH: &str = "./day02.txt";
const MAX_SAFE_LEVEL_CHANGE: u32 = 3;
const MAX_PROBLEMS_DAMPENED: usize = 1;

fn are_levels_safe(levels: Vec<u32>, mut problems_dampened: u32) {
    let mut problems_dampened: usize = 0;

    let mut i: usize = 0;
    while i < levels.len() - 1 {
        if levels[i + 1] <= levels[i] { 
            || levels[i + 1] - levels[i] > MAX_SAFE_LEVEL_CHANGE {

            if problems_dampened > 0 {
                let mut left_removed = levels.clone();
                let mut right_removed = levels.clone();
                left_removed.remove(i);
                right_removed.remove(i + 1);
                problems_dampened -= 1;

                if are_levels_safe(left_removed, problems_dampened)
                    || are_levels_safe(right_removed, problems_dampened) {
                        return true
                } else {
                    return false;
                }

            } else {
                return false;
            }
        }
    }
    }

    true
}

fn main() {
    let mut n_safe_reports: u32 = 0;

    // iterate over lines
    'outer: for line in std::fs::read_to_string(FILE_PATH).unwrap().lines() {
        let mut levels: Vec<u32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();

        if levels[0] > levels[1] { // reverse if in descending order
            levels = levels.into_iter().rev().collect();
        }

        if is_level_safe(level) {
            n_safe_reports += 1;
        }
    }

    println!("{}", n_safe_reports);
}

