use regex::Regex;

const FILE_PATH: &str = "./day03.txt";

fn main() {
    let input = &*std::fs::read_to_string(FILE_PATH).unwrap();
    let re = Regex::new(r"mul\((?P<op1>\d{1,3}),(?P<op2>\d{1,3})\)").unwrap();

    let do_indices: Vec<_> = input.match_indices("do()").map(|(i, _)| i).collect();
    let dont_indices: Vec<_> = input.match_indices("don't()").map(|(i, _)| i).collect();

    let mut mult_indices: Vec<usize> = Vec::new();

    let mut operands: Vec<(u32, u32)> = Vec::new();

    for found in re.find_iter(input) {
        mult_indices.push(found.start());

    }

    for capture in re.captures_iter(input) {
        let (_, [op1, op2]) = capture.extract();
        operands.push((op1.parse().unwrap(), op2.parse().unwrap()));
    }

    let mut operands_iter = operands.into_iter();

    let mut enabled = true;

    let mut result: u32 = 0;
    for i in 0..mult_indices[mult_indices.len() - 1] + 1 {
        if do_indices.contains(&i) {
            enabled = true;
        } else if dont_indices.contains(&i) {
            enabled = false;
        }
        if mult_indices.contains(&i) {
            let (op1, op2) = operands_iter.next().unwrap();
            if enabled {
                result += op1 * op2;
            }
        }
    }

    println!("{}", result);
}

