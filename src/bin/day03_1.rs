use regex::Regex;

const FILE_PATH: &str = "./day03.txt";

fn main() {
    let input = &*std::fs::read_to_string(FILE_PATH).unwrap();
    let re = Regex::new(r"mul\((?P<op1>\d{1,3}),(?P<op2>\d{1,3})\)").unwrap();

    let mut operands: Vec<(u32, u32)> = Vec::new();
    for (_, [op1, op2]) in re.captures_iter(input).map(|c| c.extract()) {
        operands.push((op1.parse().unwrap(), op2.parse().unwrap()));
    }

    let mut result: u32 = 0;
    for (op1, op2) in operands {
        result += op1 * op2;
    }

    println!("{}", result);
}
