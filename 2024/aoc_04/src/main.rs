use std::ops::Not;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in std::fs::read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn populate_matrix(lines: Vec<String>) -> Vec<Vec<char>> {
    let rows = lines.len();
    let cols = lines[0].len();

    let mut matrix = vec![vec!['#'; cols]; rows];

    for (i, line) in lines.iter().enumerate() {
        for (j, letter) in line.chars().enumerate() {
            matrix[i][j] = letter;
        }
    }

    return matrix;
}

fn have_xmas(lines: Vec<Vec<char>>, i: i32, j: i32, d: &(i32, i32), n: usize, m: usize) -> i32 {
    let (dx, dy) = d;

    for (k, x) in "XMAS".chars().enumerate() {
        let ii = i + (k as i32) * dx;
        let jj = j + (k as i32) * dy;
        if !(0 <= ii
            && <i32 as TryInto<usize>>::try_into(ii).unwrap() < n
            && 0 <= jj
            && jj < m.try_into().unwrap())
        {
            return 0;
        }
        if lines[ii as usize][jj as usize] != x {
            return 0;
        }
    }
    return 1;
}

fn have_mas_x(lines: Vec<Vec<char>>, i: i32, j: i32, n: usize, m: usize) -> i32 {
    if !(1 <= i
        && i < <usize as TryInto<i32>>::try_into(n).unwrap() - 1
        && 1 <= j
        && j < <usize as TryInto<i32>>::try_into(m).unwrap() - 1)
    {
        return 0;
    }
    let ii = i as usize;
    let jj = j as usize;

    if lines[ii as usize][jj] != 'A' {
        return 0;
    }

    let diag_1 = format!("{}{}", lines[ii - 1][jj - 1], lines[ii + 1][jj + 1]);
    let diag_2 = format!("{}{}", lines[ii - 1][jj + 1], lines[ii + 1][jj - 1]);

    let array: [&str; 2] = ["MS", "SM"];

    if array.contains(&diag_1.as_str()) && array.contains(&diag_2.as_str()) {
        return 1;
    }

    return 0;
}

fn main() {
    let lines = read_lines("input.txt");
    let matrix = populate_matrix(lines);

    let mut count = 0;
    let mut count2 = 0;

    let n = matrix.len();
    let m = matrix[0].len();

    let mut dd: Vec<(i32, i32)> = vec![];

    for dx in -1i32..2 {
        for dy in -1i32..2 {
            if dx != 0 || dy != 0 {
                dd.push((dx, dy))
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            for d in &dd {
                count += have_xmas(
                    matrix.clone(),
                    i.try_into().unwrap(),
                    j.try_into().unwrap(),
                    d,
                    n,
                    m,
                );
            }
            count2 += have_mas_x(
                matrix.clone(),
                i.try_into().unwrap(),
                j.try_into().unwrap(),
                n,
                m,
            );
        }
    }
    println!("{:?}", count);
    println!("{:?}", count2);
}
