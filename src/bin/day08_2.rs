use std::collections::HashSet;

const FILE_PATH: &str = "day08.txt";
const EMPTY_SQUARE_CHAR: char = '.';

#[derive(PartialEq, Clone, Copy, Debug)]
struct Antenna {
    id: char,
    pos: (isize, isize),
}

fn main() {
    let mut antennas: Vec<Antenna> = Vec::new();

    let input_str = std::fs::read_to_string(FILE_PATH).expect("Error opening input file");
    let lines = input_str.lines();
    let dims: (isize, isize) = (lines.clone().count() as isize, lines.clone().next().unwrap().len() as isize);
    let is_onscreen = |x, y| x >= 0 && x < dims.0 && y >= 0 && y < dims.1;

    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == EMPTY_SQUARE_CHAR {
                continue;
            }

            antennas.push(Antenna { id: c, pos: (x as isize, y as isize) });
        }
    }

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    for antenna1 in &antennas {
        for antenna2 in &antennas {
            if antenna1 == antenna2 || antenna1.id != antenna2.id {
                continue;
            }

            let dist = (antenna1.pos.0 - antenna2.pos.0, antenna1.pos.1 - antenna2.pos.1);

            let mut i: isize = 0;

            while is_onscreen(antenna1.pos.0 + dist.0 * i, antenna1.pos.1 + dist.1 * i) {
                antinodes.insert((antenna1.pos.0 + dist.0 * i, antenna1.pos.1 + dist.1 * i));
                i += 1;
            }

            i = 0;

            while is_onscreen(antenna2.pos.0 - dist.0 * i, antenna2.pos.1 - dist.1 * i) {
                antinodes.insert((antenna2.pos.0 - dist.0 * i, antenna2.pos.1 - dist.1 * i));
                i += 1;
            }
        }
    }

    println!("{:?}", antinodes.into_iter().filter(|&(x, y)| is_onscreen(x, y)).count());
}

