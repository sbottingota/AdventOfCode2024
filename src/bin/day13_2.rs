use regex::Regex;

const FILE_PATH: &str = "day13.txt";
const A_TOKENS: isize = 3;
const B_TOKENS: isize = 1;

const PRIZE_POS_OFFSET: isize = 10000000000000;

const EPSILON: f64 = 1e-5;

// taken from https://www.reddit.com/r/adventofcode/comments/1hd7irq/2024_day_13_an_explanation_of_the_mathematics/
fn solve(a: (isize, isize), b: (isize, isize), prize: (isize, isize)) -> Option<(isize, isize)> {
    let res = (
        (((prize.0 * b.1 - b.0 * prize.1) as f64 / (a.0 * b.1 - a.1 * b.0) as f64) + EPSILON) as isize,
        (((prize.1 * a.0 - a.1 * prize.0)as f64 / (a.0 * b.1 - a.1 * b.0) as f64) + EPSILON) as isize,
    ); 

    if (a.0 * res.0 + b.0 * res.1, a.1 * res.0 + b.1 * res.1) == prize {
        Some(res)
    } else {
        None
    }
}

fn main() {
    let re = Regex::new(r"^.+: X.(?<x>\d+), Y.(?<y>\d+)$").unwrap();

    let mut games: Vec<Vec<(isize, isize)>> = Vec::new();

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

    let mut n_tokens = 0_isize;

    for game in games {
        let a = game[0];
        let b = game[1];
        let prize = (game[2].0 + PRIZE_POS_OFFSET, game[2].1 + PRIZE_POS_OFFSET);
        
        let solutions = solve(a, b, prize);
        if solutions.is_some() {
            let (a_times, b_times) = solutions.unwrap();
            n_tokens += a_times * A_TOKENS + b_times * B_TOKENS;
        }
    }

    println!("{}", n_tokens);
}


