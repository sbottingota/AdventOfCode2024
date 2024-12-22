use itertools::Itertools;
use itertools::repeat_n;

const FILE_PATH: &str = "day22.txt";
const N_ITERATIONS: usize = 2000;
const SEQUENCE_LENGTH: usize = 4;

fn simulate_iteration(mut n: usize) -> usize {
    n = (n * 64) ^ n;
    n %= 16777216;

    n = (n / 32) ^ n;
    n %= 16777216;

    n = (n * 2048) ^ n;
    n %= 16777216;

    return n;
}

fn simulate_change_sequence(secret_numbers: &[usize], change_sequence: &[isize]) -> usize {
    let mut changes: Vec<isize> = Vec::new();

    let mut n_bananas = 0_usize;

    for mut n in secret_numbers.into_iter().cloned() {
        changes.clear();
        for _ in 0..N_ITERATIONS {
            let new = simulate_iteration(n);
            changes.insert(0, (new as isize % 10) - (n as isize % 10));
            n = new;

            if changes.len() == change_sequence.len() {
                if changes.iter().rev().zip(change_sequence.iter()).all(|(&n, &m)| n == m) {
                    n_bananas += n % 10;
                    break;
                }

                let _ = changes.pop();
            }
        }
    }

    return n_bananas;
}

fn main() {
    let secret_numbers: Vec<usize> = std::fs::read_to_string(FILE_PATH)
            .expect("Error reading input file")
            .lines()
            .map(|line| line.parse().unwrap())
            .collect();

    let mut best_sequence = vec![0; SEQUENCE_LENGTH];
    let mut best_n_bananas = 0;

    for sequence in repeat_n(-8_isize..=8, SEQUENCE_LENGTH).multi_cartesian_product() {
        let n_bananas = simulate_change_sequence(&secret_numbers, &sequence);

        if n_bananas > best_n_bananas {
            best_n_bananas = n_bananas;
            best_sequence = sequence.to_vec();
        }
    }  

    println!("{}", best_sequence.into_iter().join(","));
}

