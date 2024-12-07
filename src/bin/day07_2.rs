use itertools::Itertools;
use std::iter::zip;

type Operator = dyn Fn(u64, u64) -> u64;

const FILE_PATH: &str = "./day07.txt";

fn main() {
    let operators: Vec<Box<Operator>> = vec![Box::new(|x, y| x + y), Box::new(|x, y| x * y), Box::new(|x, y| format!("{}{}", x, y).parse().unwrap())];

    let mut test_values: Vec<u64> = Vec::new();
    let mut test_operands: Vec<Vec<u64>> = Vec::new();

    for line in std::fs::read_to_string(FILE_PATH).expect("Error opening input file").lines() {
        let mut line_iter = line.split_whitespace();

        let value_str = line_iter.next().unwrap(); // value string, still with colon at the end
        test_values.push(value_str[..value_str.len() - 1].parse().unwrap());

        let mut operands: Vec<u64> = Vec::new();
        for s in line_iter {
            operands.push(s.parse().unwrap());
        }
        test_operands.push(operands);
    }

    let mut result_sum: u64 = 0;
    for i in 0..test_values.len() {
        let operator_combinations = (0..test_operands[i].len() - 1).map(|_| 0..operators.len()).multi_cartesian_product();
        for op_combination in operator_combinations {
            let mut res: u64 = test_operands[i][0];

            for (i, n) in zip(op_combination, &test_operands[i][1..]) {
                res = operators[i](res, *n);
            }

            if res == test_values[i] {
                result_sum += res;
                break;
            }
        }
    }

    println!("{}", result_sum);
}

