const FILE_PATH: &str = "./day05.txt";

fn is_update_in_order(update: &[u32], ordering: &[(u32, u32)]) -> bool {
    for rule in ordering {
        if update.iter().position(|&n| n == rule.0).unwrap_or(0) > update.iter().position(|&n| n == rule.1).unwrap_or(std::usize::MAX) {
            return false;
        }
    }

    true
}

fn main() {
    let input = std::fs::read_to_string(FILE_PATH).expect("Could not read file");
    let lines: Vec<&str> = input.lines().collect();

    let mut ordering: Vec<(u32, u32)> = Vec::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();

    let mut end: usize = 0;

    for (i, line) in lines.iter().enumerate() {
        if line.is_empty() {
            end = i;
            break;
        }

        let mut split = line.split("|");

        ordering.push((split.next().unwrap().parse().unwrap(), split.next().unwrap().parse().unwrap()));
    }

    for line in &lines[end+1..] {
        let mut update: Vec<u32> = Vec::new();
        for s in line.split(",") {
            update.push(s.parse().unwrap());
        }

        updates.push(update);
    }

    let mut middles_sum: u32 = 0;

    for update in updates {
        if is_update_in_order(&update, &ordering) {
            middles_sum += update[update.len() / 2];
        }
    }

    println!("{}", middles_sum);
}

