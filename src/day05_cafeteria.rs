use std::fs::read_to_string;

pub struct Range {
    min: u64,
    max: u64
}

impl Range {
    pub fn validate(&self, id: u64) -> bool {
        return id >= self.min && id <= self.max;
    }
}

pub fn solve(input_path: &str) {
    let mut solution:u64 = 0;
    let input = read_to_string(input_path)
        .expect("failed to read input");

    let mut ranges: Vec<Range> = vec![];
    for line in input.lines() {
        if line == "" { continue; }
        if line.contains("-") {
            let bounds:Vec<&str> = line.split("-").collect();
            if bounds.len() != 2 { panic!("error parsing range"); }
            ranges.push(Range { min: bounds[0].parse().expect("error parsing min range"), max: bounds[1].parse().expect("error parsing max range") });
        } else {
            let id:u64 = line.parse().expect("error parsing id"); 
            let mut valid = false;
            for range in ranges.iter() {
                if range.validate(id) {
                    valid = true;
                    break;
                }
            }
            if valid {
                solution += 1;
            }

        }
    }
    println!("Day 05 - Cafeteria\n  - Solution: {}\n", solution);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample() {
    }
}