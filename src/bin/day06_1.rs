use std::collections::HashSet;

const FILE_PATH: &str = "./day06.txt";

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn right(&self) -> Direction {
        Direction::from_usize((*self as usize + 1) % 4)
    }

    // return (y, x), because that's how the grid is stored
    fn get_facing_coords(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Right => (0, 1),
            Direction::Left => (0, -1),
        }
    }

    fn from_usize(n: usize) -> Direction {
        match n {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            x => panic!("Invalid input {}", x)
        }
    }
}

#[derive(PartialEq, Debug)]
enum Square {
    Empty,
    Guard { direction: Direction },
    Obsticle,
}

impl Square {
    fn get_direction(&self) -> Direction {
        if let Square::Guard { direction } = self {
            *direction
        } else {
            panic!("Invalid square '{:?}'", self);
        }
    }
}

#[derive(Debug)]
struct Board {
    grid: Vec<Vec<Square>>,
    guard_pos: (isize, isize),
    grid_dimensions: (usize, usize),
}

impl Board {
    fn from_string(s: &str) -> Board {
        let mut board = Board { grid: Vec::new(), guard_pos: (0, 0), grid_dimensions: (0, 0) };
        for (i, line) in s.to_string().lines().enumerate() {
            board.grid.push(Vec::new());

            for (j, c) in line.chars().enumerate() {
                board.grid[i].push(
                    match c {
                        '.' => Square::Empty,
                        '^' => {
                             board.guard_pos = (i as isize, j as isize);
                             Square::Guard {direction: Direction::Up }
                        },
                        '#' => Square::Obsticle,
                        _ => panic!("Invalid character found when parsing board string")
                    }
                );
            }

        }
        board.grid_dimensions = (board.grid.len(), board.grid[0].len());

        board
    }

    fn move_guard(&mut self) {
        let guard_direction = self.grid[self.guard_pos.0 as usize][self.guard_pos.1 as usize].get_direction();
        let facing = guard_direction.get_facing_coords();
        let new_pos = (self.guard_pos.0 + facing.0, self.guard_pos.1 + facing.1);

        if new_pos.0 < 0 || new_pos.0 >= self.grid_dimensions.0 as isize || new_pos.1 < 0 || new_pos.1 >= self.grid_dimensions.1 as isize {
            self.guard_pos = new_pos;
            return;
        }

        if self.grid[new_pos.0 as usize][new_pos.1 as usize] != Square::Empty {
            self.grid[self.guard_pos.0 as usize][self.guard_pos.1 as usize] = Square::Guard { direction: guard_direction.right() };
        } else {
            self.grid[new_pos.0 as usize][new_pos.1 as usize] = Square::Guard { direction: guard_direction };
            self.grid[self.guard_pos.0 as usize][self.guard_pos.1 as usize] = Square::Empty;
            self.guard_pos = new_pos;
        }
    }

    fn is_guard_gone(&self) -> bool {
        self.guard_pos.0 >= self.grid_dimensions.0 as isize || self.guard_pos.0 < 0 || self.guard_pos.1 >= self.grid_dimensions.1 as isize || self.guard_pos.1 < 0
    }
}

fn main() {
    let mut board = Board::from_string(&std::fs::read_to_string(FILE_PATH).expect("Failed to open input file"));

    let mut guard_positions: HashSet<(isize, isize)> = HashSet::new();

    while !board.is_guard_gone() {
        guard_positions.insert(board.guard_pos);
        board.move_guard();
    }

    println!("{}", guard_positions.len());
}

