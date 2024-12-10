use std::collections::VecDeque;

const FILE_PATH: &str = "day10.txt";

const START: u32 = 0;
const END: u32 = 9;

fn get_trail_count(grid: &Vec<Vec<u32>>, pos: (usize, usize)) -> usize {
    let mut next_moves: VecDeque<(usize, usize)> = VecDeque::new();
    next_moves.push_back(pos);

    let dims = (grid.len(), grid[0].len());

    let mut n_trails = 0_usize;

    while next_moves.is_empty() {
        let next_move = next_moves.pop_front().unwrap();
        if grid[next_move.0][next_move.1] == END {
            n_trails += 1;
            continue;
        }

        if next_move.0 > 0 && grid[next_move.0 - 1][next_move.1] == grid[next_move.0][next_move.1] + 1 {
            next_moves.push_back((next_move.0 - 1, next_move.1));
        }
        if next_move.1 > 0 && grid[next_move.0][next_move.1 - 1] == grid[next_move.0][next_move.1] + 1 {
            next_moves.push_back((next_move.0, next_move.1 - 1));
        }
        if next_move.0 < dims.0 - 1 && grid[next_move.0 + 1][next_move.1] == grid[next_move.0][next_move.1] + 1 {
            next_moves.push_back((next_move.0 + 1, next_move.1));
        }
        if next_move.1 < dims.1 - 1 && grid[next_move.0][next_move.1 + 1] == grid[next_move.0][next_move.1] + 1 {
            next_moves.push_back((next_move.0, next_move.1 + 1));
        }
    }

    n_trails
}

fn main() {
    let mut grid: Vec<Vec<u32>> = Vec::new();

    for line in std::fs::read_to_string(FILE_PATH).expect("Error reading input file").lines() {
        grid.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    let mut trail_count = 0_usize;
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == START {
                trail_count += get_trail_count(&grid, (x, y));
            }
        }
    }

    println!("{}", trail_count);
}

