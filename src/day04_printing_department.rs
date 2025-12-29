use std::fs::read_to_string;

pub fn get_valid_papers(input: &str) -> (u64, String) {
    let mut map:Vec<Vec<bool>> = vec![];
    let mut next_map = "".to_string();
    input.lines().for_each(|line| {
        map.push(line.chars().map(|slot| { slot == '@' }).collect());
    });
    let map = map;

    let mut result = 0;
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] && validate_spot(&map, row, col) {
                result += 1;
                next_map.push('.');
            } else if map[row][col] {
                next_map.push('@');
            } else {
                next_map.push('.');
            }
        }
        next_map.push('\n');
    }

    return (result, next_map.to_string());
}

fn validate_spot(map: &Vec<Vec<bool>>, row: usize, col: usize) -> bool {
    let mut results: Vec<bool> = vec![];
    let mut start_row = 0;
    let mut end_row = map.len()-1;
    if row > 0 {
        start_row = row - 1;
    }
    if row < end_row {
        end_row = row + 1;
    }
    let mut start_col = 0;
    let mut end_col = map[0].len()-1;
    if col > 0 {
        start_col = col - 1;
    }
    if col < end_col {
        end_col = col + 1;
    }
    for r in start_row..=end_row {
        for c in start_col..=end_col {
            if !(r == row && c == col) {
                results.push(map[r][c]);
            }
        }
    }
    return results.iter().filter(|result| {**result}).count() < 4;
}

pub fn solve(input_path: &str) {
    let mut solution:u64 = 0;
    let input = read_to_string(input_path)
        .expect("failed to read input");
    let mut map = input;

    loop {
        let (result, next_map) = get_valid_papers(map.as_str()); 
        if result == 0 {
            break;
        }
        solution += result;
        map = next_map;
    }

    println!("Day 04 - Printing Department\n  - Solution: {}\n", solution);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample() {
        let (solution, _) = get_valid_papers("..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.");
        assert_eq!(13, solution);
    }
}