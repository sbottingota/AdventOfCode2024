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

fn main() {
    let mut grid = [[State::Safe; GRID_SIZE]; GRID_SIZE]; 
    
    for line in std::fs::read_to_string(FILE_PATH).expect("Error reading input file").lines().take(N_ITERATIONS) {
        let pos: Vec<usize> = line.split(',').map(|substr| substr.parse().unwrap()).collect();
        grid[pos[0]][pos[1]] = State::Corrupted;
    }

    // (pos, path len so far)
    let mut next_positions: VecDeque<([usize; 2], usize)> = VecDeque::new();
    next_positions.push_back((START_SQUARE, 0));

    let mut visited: HashSet<[usize; 2]> = HashSet::new();
    visited.insert(START_SQUARE);

    loop {
        let (current_pos, current_len) = next_positions.pop_front().unwrap();
        let possible_moves = get_possible_moves_from(current_pos);

        for pos in possible_moves {
            if grid[pos[0]][pos[1]] == State::Safe && !visited.contains(&pos) {
                if pos == TARGET_SQUARE {
                    println!("{}", current_len + 1);
                    return;
                } else {
                    next_positions.push_back((pos, current_len + 1));
                    visited.insert(pos);
                }
            }
        }
    }
}

