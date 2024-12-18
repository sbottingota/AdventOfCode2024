use std::collections::{VecDeque, HashSet};

const FILE_PATH: &str = "day18.txt";
const GRID_SIZE: usize = 71;

const N_ITERATIONS: usize = 1024;

const START_SQUARE: [usize; 2] = [0, 0];
const TARGET_SQUARE: [usize; 2] = [GRID_SIZE - 1, GRID_SIZE - 1];

#[derive(PartialEq, Clone, Copy, Debug)]
enum State {
    Safe,
    Corrupted,
}

fn get_possible_moves_from(pos: [usize; 2]) -> Vec<[usize; 2]> {
    let mut possible_moves = Vec::new();

    if pos[0] > 0 {
        possible_moves.push([pos[0] - 1, pos[1]]);
    }
    if pos[0] < GRID_SIZE - 1 {
        possible_moves.push([pos[0] + 1, pos[1]]);
    }

    if pos[1] > 0 {
        possible_moves.push([pos[0], pos[1] - 1]);
    }
    if pos[1] < GRID_SIZE - 1 {
        possible_moves.push([pos[0], pos[1] + 1]);
    }


    possible_moves
}

fn path_to_exit_exists(grid: &[[State; GRID_SIZE]; GRID_SIZE]) -> bool {
    let mut next_positions: VecDeque<[usize; 2]> = VecDeque::new();
    next_positions.push_back(START_SQUARE);

    let mut visited: HashSet<[usize; 2]> = HashSet::new();
    visited.insert(START_SQUARE);

    while !next_positions.is_empty() {
        let current_pos = next_positions.pop_front().unwrap();
        let possible_moves = get_possible_moves_from(current_pos);

        for pos in possible_moves {
            if grid[pos[0]][pos[1]] == State::Safe && !visited.contains(&pos) {
                if pos == TARGET_SQUARE {
                    return true;
                } else {
                    next_positions.push_back(pos);
                    visited.insert(pos);
                }
            }
        }
    }

    false
}

fn main() {
    let mut grid = [[State::Safe; GRID_SIZE]; GRID_SIZE]; 
    let input_string = std::fs::read_to_string(FILE_PATH).expect("Error reading input file");
    let mut positions_iter = input_string.lines().map(|line| line.split(',').map(|substr| substr.parse::<usize>().unwrap()).collect::<Vec<_>>());

    for pos in positions_iter.by_ref().take(N_ITERATIONS) {
        grid[pos[0]][pos[1]] = State::Corrupted;
    }

    for pos in positions_iter {
        grid[pos[0]][pos[1]] = State::Corrupted;
        if !path_to_exit_exists(&grid) {
            println!("{},{}", pos[0], pos[1]);
            return;
        }
    }
}

