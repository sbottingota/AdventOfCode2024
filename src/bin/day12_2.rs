use std::collections::HashSet;

const FILE_PATH: &str = "day12.txt";

fn calculate_plot_from(grid: &[Vec<char>], pos: (usize, usize)) -> HashSet<(usize, usize)> {
    let dims = (grid.len(), grid[0].len());
    let plot_type = grid[pos.0][pos.1];

    let mut plot = HashSet::new();
    plot.insert(pos);

    loop {
       let mut new_plot = plot.clone();

        for square in &plot {
            if square.0 > 0 && grid[square.0 - 1][square.1] == plot_type {
                new_plot.insert((square.0 - 1, square.1));
            }
            if square.0 < dims.0 - 1 && grid[square.0 + 1][square.1] == plot_type {
                new_plot.insert((square.0 + 1, square.1));
            }
            if square.1 > 0 && grid[square.0][square.1 - 1] == plot_type {
                new_plot.insert((square.0, square.1 - 1));
            }
            if square.1 < dims.1 - 1 && grid[square.0][square.1 + 1] == plot_type {
                new_plot.insert((square.0, square.1 + 1));
            }
        }

        if plot == new_plot {
            return new_plot;
        } else {
            plot = new_plot;
        }
    }
}

fn main() {
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in std::fs::read_to_string(FILE_PATH).expect("Error reading input file").lines() {
        grid.push(line.chars().collect());
    }

    let mut plots: Vec<HashSet<(usize, usize)>> = Vec::new();

    for x in 0..grid.len() {
        for y in 0..grid.len() {
            if plots.iter().any(|plot| plot.contains(&(x, y))) {
                continue;
            }

            plots.push(calculate_plot_from(&grid, (x, y)));
        }
    }

    let mut total_cost = 0_usize;

    for plot in plots {
        let mut n_corners = 0_usize;

        for square in &plot {
            if (square.0 == 0 || !plot.contains(&(square.0 - 1, square.1)))
                && (square.1 == 0 || !plot.contains(&(square.0, square.1 - 1))) {
                n_corners += 1;
            }
            if !plot.contains(&(square.0 + 1, square.1))
                && (square.1 == 0 || !plot.contains(&(square.0, square.1 - 1))) {
                n_corners += 1;
            }
            if (square.0 == 0 || !plot.contains(&(square.0 - 1, square.1)))
                && !plot.contains(&(square.0, square.1 + 1)) {
                n_corners += 1;
            }
            if !plot.contains(&(square.0 + 1, square.1))
                && !plot.contains(&(square.0, square.1 + 1)) {
                n_corners += 1;
            }

            if !(square.0 == 0 || !plot.contains(&(square.0 - 1, square.1)))
                && !(square.1 == 0 || !plot.contains(&(square.0, square.1 - 1)))
                && !plot.contains(&(square.0 - 1, square.1 - 1)) {
                n_corners += 1;
            }
            if plot.contains(&(square.0 + 1, square.1))
                && !(square.1 == 0 || !plot.contains(&(square.0, square.1 - 1))) 
                && !plot.contains(&(square.0 + 1, square.1 - 1)) {
                n_corners += 1;
            }
            if !(square.0 == 0 || !plot.contains(&(square.0 - 1, square.1)))
                && plot.contains(&(square.0, square.1 + 1))
                && !plot.contains(&(square.0 - 1, square.1 + 1)) {
                n_corners += 1;
            }
            if plot.contains(&(square.0 + 1, square.1))
                && plot.contains(&(square.0, square.1 + 1))
                && !plot.contains(&(square.0 + 1, square.1 + 1)) {
                n_corners += 1;
            }
        }

        total_cost += plot.len() * n_corners;
    }

    println!("{}", total_cost);
}

