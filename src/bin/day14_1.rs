use regex::Regex;

const FILE_PATH: &str = "day14.txt";

const ROOM_WIDTH: isize = 101; 
const ROOM_HEIGHT: isize = 103;

const N_ITERATIONS: usize = 100;

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

    for _ in 0..N_ITERATIONS {
        for robot in &mut robots {
            robot.step();
        }
    }

    let mut quadrants = [0_usize; 4];
    for robot in &robots {
        if robot.x < ROOM_WIDTH / 2 {
            if robot.y < ROOM_HEIGHT / 2 {
                quadrants[0] += 1;
            } else if robot.y > ROOM_HEIGHT / 2 {
                quadrants[1] += 1;
            }
        } else if robot.x > ROOM_WIDTH / 2 {
            if robot.y < ROOM_HEIGHT / 2 {
                quadrants[2] += 1;
            } else if robot.y > ROOM_HEIGHT / 2 {
                quadrants[3] += 1;
            }
        }
    }

    println!("{}", quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]);
}

