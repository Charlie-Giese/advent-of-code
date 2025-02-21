use helper::{count_cols, count_lines, read_lines};
use std::fs::File;
use std::io;

enum Direction {
    North,
    South,
    East,
    West,
}

struct Grid {
    nrows: usize,
    ncols: usize,
    current_position: (usize, usize),
    current_direction: Direction,
    map: Vec<char>, // Using a flattened vector: (row, col) -> row * ncols + col
}

impl Direction {
    fn turn(&self) -> Self {
        use Direction::*;
        match *self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}

impl Grid {
    fn load_lines(&mut self, lines: io::Lines<io::BufReader<File>>) {
        for line in lines {
            for c in line.unwrap().chars() {
                self.map.push(c);
            }
        }
    }
}

fn main() {
    let filename = "input.txt";

    let lines = read_lines(filename);
    let nrows = count_lines(filename);
    let ncols = count_cols(filename);

    let mut grid = Grid {
        nrows: nrows,
        ncols: ncols,
        current_position: (0, 0),
        current_direction: Direction::North,
        map: Vec::new(),
    };

    grid.load_lines(lines.unwrap());

    println!("{:?}", grid.map);
}
