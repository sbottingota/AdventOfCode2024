use regex::Regex;

const FILE_PATH: &str = "day13.txt";
const LIMIT: usize = 100;

const A_TOKENS: usize = 3;
const B_TOKENS: usize = 1;

fn main() {
    let re = Regex::new(r"^.+: X.(?<x>\d+), Y.(?<y>\d+)$").unwrap();

    let mut games: Vec<Vec<(usize, usize)>> = Vec::new();

    for chunk in std::fs::read_to_string(FILE_PATH).expect("Error reading to input file").split("\n\n") {
        let mut game = Vec::new();
        for line in chunk.lines() {
            let Some(caps) = re.captures(line) else {
                panic!("Error parsing line: '{}'", line);
            };
            game.push((caps["x"].parse().unwrap(), caps["y"].parse().unwrap()));
        }
        games.push(game);
    }

    let mut n_tokens = 0_usize;

    'outer: for game in games {
        let a = game[0];
        let b = game[1];
        let prize = game[2];
        
        for a_times in 0..LIMIT {
            for b_times in 0..LIMIT {
                if a.0 * a_times + b.0 * b_times == prize.0
                    && a.1 * a_times + b.1 * b_times == prize.1 {
                    n_tokens += a_times * A_TOKENS + b_times * B_TOKENS;
                    continue 'outer;
                }
            }
        }
    }

    println!("{}", n_tokens);
}


