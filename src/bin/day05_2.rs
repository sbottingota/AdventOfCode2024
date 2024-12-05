const FILE_PATH: &str = "./day05.txt";

fn get_update_problems(update: &Vec<u32>, ordering: &[(u32, u32)]) -> Vec<(usize, usize)> {
    let mut problems: Vec<(usize, usize)> = Vec::new();

    for rule in ordering {
        let pos1 = update.iter().position(|&n| n == rule.0).unwrap_or(0);
        let pos2 = update.iter().position(|&n| n == rule.1).unwrap_or(std::usize::MAX);
        if pos1 > pos2 {
            problems.push((pos1, pos2));
        }
    }

    problems.sort_by(|(i, _), (j, _)| i.cmp(j));
    problems
}

// returns fixed vector if needed fixing, else none
fn fix_update_order(mut update: Vec<u32>, ordering: &[(u32, u32)]) -> Vec<u32> {
    loop {
        let problems = get_update_problems(&update, ordering);

        if problems.len() == 0 {
            return update;
        }

        let temp = update[problems[0].0];
        update[problems[0].0] = update[problems[0].1];
        update[problems[0].1] = temp;
    }
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
        let fixed = fix_update_order(update.clone(), &ordering);
        if fixed != update {
            middles_sum += fixed[update.len() / 2];
        }
    }

    println!("{}", middles_sum);
}
