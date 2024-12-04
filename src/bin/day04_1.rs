const FILE_PATH: &str = "./day04.txt";
const MATCH_LEN: usize = 4;

fn main() {
    let input = std::fs::read_to_string(FILE_PATH).expect("Could not read input file");
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    let height = grid.len();
    let width = grid[0].len();

    let mut combinations: Vec<Vec<char>> = Vec::new(); 

    for y in 0..height {
        for x in 0..width {
            if width - x >= MATCH_LEN {
                combinations.push(grid[y][x..x+MATCH_LEN].to_vec());
                combinations.push(grid[y][x..x+MATCH_LEN].iter().map(|&x| x).rev().collect());
            }

            if height - y >= MATCH_LEN {
                let vec = vec![grid[y][x], grid[y+1][x], grid[y+2][x], grid[y+3][x]];
                combinations.push(vec.iter().map(|&x| x).rev().collect());
                combinations.push(vec);
            }

            if width - x >= MATCH_LEN && height - y >= MATCH_LEN {
                let vec = vec![grid[y][x], grid[y+1][x+1], grid[y+2][x+2], grid[y+3][x+3]];
                combinations.push(vec.iter().map(|&x| x).rev().collect());
                combinations.push(vec);
            }

            if x >= MATCH_LEN - 1 && height - y >= MATCH_LEN {
                let vec = vec![grid[y][x], grid[y+1][x-1], grid[y+2][x-2], grid[y+3][x-3]];
                combinations.push(vec.iter().map(|&x| x).rev().collect());
                combinations.push(vec);
            }
        }
    }

    let mut count: usize = 0;

    for combination in combinations {
        if combination == "XMAS".chars().collect::<Vec<char>>() {
            count += 1;
        }
    }

    println!("{}", count);
}

