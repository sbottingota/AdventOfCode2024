use std::collections::{VecDeque, HashSet};
use std::hash::{Hash, Hasher};

const FILE_PATH: &str = "day20.txt";
const THRESHOLD: usize = 100;

#[derive(PartialEq)]
enum Cell {
    Empty,
    Wall,   
    End,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' | 'S' => Self::Empty,
            '#' => Self::Wall,
            'E' => Self::End,
            _ => panic!("Tried to convert invalid character '{}' into Cell", c),
        }
    }
}

#[derive(Clone, Eq, Debug)]
struct State {
    x: usize,
    y: usize,
    cost: usize,
}

impl State {
    fn next_states(&self, grid: &[Vec<Cell>]) -> Vec<Self> {
        let mut states = Vec::new();

        if self.x > 0 && grid[self.x - 1][self.y] != Cell::Wall {
            states.push(State { x: self.x - 1, y: self.y, cost: self.cost + 1 });
        }

        if self.y > 0 && grid[self.x][self.y - 1] != Cell::Wall {
            states.push(State { x: self.x, y: self.y - 1, cost: self.cost + 1 });
        }

        if self.x < grid.len() - 1 && grid[self.x + 1][self.y] != Cell::Wall {
            states.push(State { x: self.x + 1, y: self.y, cost: self.cost + 1 });
        }

        if self.y < grid[0].len() - 1 && grid[self.x][self.y + 1] != Cell::Wall {
            states.push(State { x: self.x, y: self.y + 1, cost: self.cost + 1 });
        }

        states
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

fn get_shortest_path_len(grid: &[Vec<Cell>], start: &State) -> usize {
    let mut to_visit = VecDeque::from([start.clone()]);
    let mut seen = HashSet::from([start.clone()]);

    while let Some(state) = to_visit.pop_front() {
        for next in state.next_states(&grid) {
            if grid[next.x][next.y] == Cell::End {
                return next.cost;
            } else if !seen.contains(&next) {
                seen.insert(next.clone());
                to_visit.push_back(next);
            }
        }
    }

    panic!("No shortest path to end found");
}

fn main() {
    let mut grid: Vec<Vec<Cell>> = Vec::new();

    let mut start = State { x: 0, y: 0, cost: 0 }; // dummy position values

    for (x, line) in std::fs::read_to_string(FILE_PATH).expect("Error reading input file").lines().enumerate() {
        grid.push(Vec::new());
        for (y, c) in line.chars().enumerate() {
            grid.last_mut().unwrap().push(Cell::from(c));
            if c == 'S' {
                start.x = x;
                start.y = y;
            }
        }
    }

    let baseline = get_shortest_path_len(&grid, &start);

    let mut count = 0_usize;

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] != Cell::Wall {
                continue;
            }

            grid[x][y] = Cell::Empty;
            if get_shortest_path_len(&grid, &start) + THRESHOLD <= baseline {
                count += 1;
            }
            grid[x][y] = Cell::Wall;
        }
    }

    println!("{}", count);
}

