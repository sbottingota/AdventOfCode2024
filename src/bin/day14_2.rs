use regex::Regex;

const FILE_PATH: &str = "day14.txt";

const ROOM_WIDTH: isize = 101; 
const ROOM_HEIGHT: isize = 103;

#[derive(Debug)]
struct Robot {
    x: isize,
    y: isize,
    x_speed: isize,
    y_speed: isize,
}

impl Robot {
    fn step(&mut self) {
        self.x = (self.x + self.x_speed + ROOM_WIDTH) % ROOM_WIDTH;
        self.y = (self.y + self.y_speed + ROOM_HEIGHT) % ROOM_HEIGHT;
    }
}

fn looks_like_christmas_tree(robots: &[Robot]) -> bool {
    for robot in robots {
        if robots.iter().filter(|&other| other.x == robot.x && other.y == robot.y).count() > 1 {
            return false;
        }
    }

    true
}

fn main() {
    let re = Regex::new(r"p=(?<x>-?\d+),(?<y>-?\d+) v=(?<dx>-?\d+),(?<dy>-?\d+)").unwrap();
    
    let mut robots: Vec<Robot> = Vec::new();
    for line in std::fs::read_to_string(FILE_PATH).expect("Error reading input file").lines() {
        let Some(caps) = re.captures(line) else {
            panic!("Error parsing line {}", line);
        };

        robots.push(Robot { 
            x: caps["x"].parse().unwrap(),
            y: caps["y"].parse().unwrap(),
            x_speed: caps["dx"].parse().unwrap(),
            y_speed: caps["dy"].parse().unwrap(),
        });
    }

    let mut i = 1_usize;
    loop {
       for robot in &mut robots {
            robot.step();
        }

        if looks_like_christmas_tree(&robots) {
            let mut grid = [["."; ROOM_WIDTH as usize]; ROOM_HEIGHT as usize];
            for robot in &robots {
                grid[robot.y as usize][robot.x as usize] = "x";
            }

            println!("{}", i);
            for row in grid {
                println!("{}", row.join(""));
            }
            return;
        }

        i += 1;
    }
}
