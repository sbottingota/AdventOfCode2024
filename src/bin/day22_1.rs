const FILE_PATH: &str = "day22.txt";
const N_ITERATIONS: usize = 2000;

fn simulate_n_iterations(mut n: usize, n_iterations: usize) -> usize {
    for _ in 0..n_iterations {
        n = (n * 64) ^ n;
        n %= 16777216;

        n = (n / 32) ^ n;
        n %= 16777216;

        n = (n * 2048) ^ n;
        n %= 16777216;
    }

    return n;
}

fn main() {
    let mut sum = 0_usize;
    for line in std::fs::read_to_string(FILE_PATH).expect("Error reading input file").lines() {
        sum += simulate_n_iterations(line.trim().parse().unwrap(), N_ITERATIONS);
    }

    println!("{}", sum);
}

