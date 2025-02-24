use helper::{count_cols, count_lines, read_lines};
use std::fs::File;
use std::io;

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

struct Grid {
    nrows: i32,
    ncols: i32,
    current_position: (i32, i32),
    current_direction: Direction,
    map: Vec<Vec<char>>,
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
    fn initialise(&mut self, lines: io::Lines<io::BufReader<File>>) {
        for line in lines {
            self.map.push(line.unwrap().chars().collect());
        }

        for (row, line) in self.map.iter().enumerate() {
            if let Some(col) = line
                .iter()
                .position(|&c| c == '^' || c == '<' || c == '>' || c == 'v')
            {
                self.current_position = (row as i32, col as i32);
                let direction = match line[col] {
                    '^' => Direction::North,
                    '>' => Direction::East,
                    '<' => Direction::West,
                    'v' => Direction::South,
                    _ => Direction::North,
                };
                self.current_direction = direction;
            }
        }
    }

    fn turn_direction(&mut self) {
        self.current_direction = self.current_direction.turn();
    }

    fn solve(&mut self) -> Option<usize> {
        while self.current_position.0 > 0
            && self.current_position.0 < self.nrows - 1
            && self.current_position.1 > 0
            && self.current_position.1 < self.ncols - 1
        {
            println!("{:?}", self.current_position);
            println!("{:?}", self.current_direction);
            println!(
                "{:?}",
                self.map[self.current_position.0 as usize][self.current_position.1 as usize]
            );
            self.map[self.current_position.0 as usize][self.current_position.1 as usize] = 'X';
            match self.current_direction {
                Direction::North => {
                    if self.map[self.current_position.0 as usize - 1]
                        [self.current_position.1 as usize]
                        == '#'
                    {
                        self.turn_direction();
                        self.current_position.1 += 1;
                    } else {
                        self.current_position.0 += -1;
                    }
                }
                Direction::East => {
                    if self.map[self.current_position.0 as usize]
                        [self.current_position.1 as usize + 1]
                        == '#'
                    {
                        self.turn_direction();
                        self.current_position.0 += 1;
                    } else {
                        self.current_position.1 += 1;
                    }
                }
                Direction::South => {
                    if self.map[self.current_position.0 as usize + 1]
                        [self.current_position.1 as usize]
                        == '#'
                    {
                        self.turn_direction();
                        self.current_position.1 += -1;
                    } else {
                        self.current_position.0 += 1;
                    }
                }
                Direction::West => {
                    if self.map[self.current_position.0 as usize]
                        [self.current_position.1 as usize - 1]
                        == '#'
                    {
                        self.turn_direction();
                        self.current_position.0 += -1;
                    } else {
                        self.current_position.1 += -1;
                    }
                }
            }
        }
        let count = self
            .map
            .iter()
            .map(|row| row.iter().filter(|&&c| c == 'X').count() as usize)
            .sum::<usize>();
        Some(count + 1)
    }
}

fn main() {
    let filename = "input.txt";

    let lines = read_lines(filename);
    let nrows = count_lines(filename);
    let ncols = count_cols(filename);

    let mut grid = Grid {
        nrows: nrows as i32,
        ncols: ncols as i32,
        current_position: (0, 0),
        current_direction: Direction::North,
        map: Vec::new(),
    };

    grid.initialise(lines.unwrap());

    let result = grid.solve();
    println!("{:?}", result.unwrap());
}
