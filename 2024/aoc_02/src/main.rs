fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in std::fs::read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn string_to_vec(string: String) -> (Vec<i32>, Vec<i32>) {
    let values = string
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let reversed = values.iter().copied().rev().collect::<Vec<i32>>();

    (values, reversed)
}

fn assert_safety(level: &Vec<i32>, reversed: &Vec<i32>) -> bool {
    let safety = (level.is_sorted()
        && (level
            .windows(2)
            .all(|w| (w[0] - w[1]).abs() < 4 && w[0] != w[1]))
        || reversed.is_sorted()
            && (reversed
                .windows(2)
                .all(|w| (w[0] - w[1]).abs() < 4 && w[0] != w[1])));

    safety
}

fn problem_dampener(level: Vec<i32>, reversed: Vec<i32>) -> bool {
    let mut safety = false;

    for i in 0..level.len() {
        let mut level_temp = level.clone();
        let mut rev_temp = reversed.clone();

        level_temp.remove(i);
        rev_temp.remove(i);

        match assert_safety(&level_temp, &rev_temp) {
            true => {
                safety = true;
                println!("{:?}", level_temp);
                println!("{:?}", rev_temp);
                println!("{:?}", rev_temp[3] - rev_temp[2]);
                println!("problem dampened");
                break;
            }
            false => {
                safety = false;
            }
        }
    }

    safety
}

fn main() {
    let lines = read_lines("input.txt");

    let mut count = 0;

    for string in lines.iter() {
        let (level, reversed) = string_to_vec(string.clone());
        if assert_safety(&level, &reversed) {
            count += 1
        } else {
            if problem_dampener(level.clone(), reversed.clone()) {
                count += 1;
            }
        }
    }

    println!("{:?}", count);
}
