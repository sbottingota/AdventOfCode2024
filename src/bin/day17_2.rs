const FILE_PATH: &str = "day17.txt";

struct ProgramValues {
    instruction_pointer: usize,
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    output: Vec<usize>,
}

impl ProgramValues {
    fn new(reg_a: usize, reg_b: usize, reg_c: usize) -> Self {
        Self { reg_a, reg_b, reg_c, instruction_pointer: 0, output: Vec::new() }
    }
}

fn get_combo_op_value(op: usize, values: &ProgramValues) -> usize {
    match op {
        0..=3 => op,
        4 => values.reg_a,
        5 => values.reg_b,
        6 => values.reg_c,
        7.. => panic!("Invalid combo operand"),
    }
}

fn adv(op: usize, values: &mut ProgramValues) {
    values.reg_a /= 2_usize.pow(get_combo_op_value(op, values) as u32);
    values.instruction_pointer += 2;
}

fn bxl(op: usize, values: &mut ProgramValues) {
    values.reg_b ^= op;
    values.instruction_pointer += 2;
}

fn bst(op: usize, values: &mut ProgramValues) {
    values.reg_b = get_combo_op_value(op, values) % 8;
    values.instruction_pointer += 2;
}

fn jnz(op: usize, values: &mut ProgramValues) {
    if values.reg_a != 0 {
        values.instruction_pointer = op;
    } else {
        values.instruction_pointer += 2;
    }
}

fn bxc(_op: usize, values: &mut ProgramValues) {
    values.reg_b ^= values.reg_c;
    values.instruction_pointer += 2;
}

fn out(op: usize, values: &mut ProgramValues) {
    values.output.push(get_combo_op_value(op, values) % 8);
    values.instruction_pointer += 2;
}

fn bdv(op: usize, values: &mut ProgramValues) {
    values.reg_b = values.reg_a / 2_usize.pow(get_combo_op_value(op, values) as u32);
    values.instruction_pointer += 2;
}

fn cdv(op: usize, values: &mut ProgramValues) {
    values.reg_c = values.reg_a / 2_usize.pow(get_combo_op_value(op, values) as u32);
    values.instruction_pointer += 2;
}

fn main() {
    let mut input: Vec<String> = Vec::new();

    for line in std::fs::read_to_string(FILE_PATH).expect("Error opening input file").lines() {
        if line.is_empty() {
            continue;
        }

        input.push(line.split(':').nth(1).unwrap().trim().to_string());
    }

    let mut reg_a: usize = 0;
    let reg_b: usize = input[1].parse().unwrap();
    let reg_c: usize = input[2].parse().unwrap();

    let program: Vec<usize> = input[3].split(',').map(|substr| substr.parse().unwrap()).collect();
    let instructions: Vec<&dyn Fn(usize, &mut ProgramValues)> = vec![&adv, &bxl, &bst, &jnz, &bxc, &out, &bdv, &cdv];

    let mut values = ProgramValues::new(reg_a, reg_b, reg_c);

    for i in (0..program.len()).rev() {
        while values.output.len() <= i || program[i] != values.output[i] {
            reg_a += 8_usize.pow(i as u32);
            values = ProgramValues::new(reg_a, reg_b, reg_c);

            while values.output.len() <= i {
                instructions[program[values.instruction_pointer]](program[values.instruction_pointer + 1], &mut values);
            }
        }
    }

    println!("{}", reg_a);
}

