use std::fs::read_to_string;

pub fn solve(input_path: &str) {
    let mut solution:u64 = 0;
    read_to_string(input_path)
        .expect("failed to read input")
        .lines()
        .for_each(|line| {
        });
    println!("Day 07 - Laboratories\n  - Solution: {}\n", solution);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample() {
    }
}