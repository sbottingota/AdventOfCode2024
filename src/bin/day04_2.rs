const FILE_PATH: &str = "./day04.txt";
const MATCH_LEN: usize = 2;

fn main() {
    let input = std::fs::read_to_string(FILE_PATH).expect("Could not read input file");
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    let height = grid.len();
    let width = grid[0].len();

    let mut combinations: Vec<Vec<char>> = Vec::new();

    for y in 0..height - MATCH_LEN {
        for x in 0..width - MATCH_LEN {
            combinations.push(vec![grid[y][x], grid[y+1][x+1], grid[y+2][x+2], grid[y+2][x], grid[y][x+2]]); 
        }
    }

    let mut count: usize = 0;

    for combination in combinations {
        if (combination[0..3] == ['M', 'A', 'S'] || combination[0..3] == ['S', 'A', 'M'])
            && (combination[3..5] == ['M', 'S'] || combination[3..5] == ['S', 'M']) {
            count += 1;
        }
    }

    println!("{}", count);
}

