use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn count_lines<P>(filename: P) -> usize
where
    P: AsRef<Path>,
{
    let input = File::open(filename).unwrap();
    let buffered = BufReader::new(input);
    let line_count = buffered.lines().count();

    line_count
}

pub fn count_cols<P>(filename: P) -> usize
where
    P: AsRef<Path>,
{
    let input = File::open(filename).unwrap();
    let buffered = BufReader::new(input);
    let col_count = buffered.lines().next().unwrap().unwrap().len();

    col_count
}
