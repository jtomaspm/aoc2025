use std::{fs::read_to_string};

pub struct Range {
    min: u64,
    max: u64
}
impl Range {
    pub fn intercepts(&self, other: &Range) -> bool {
        return self.min <= other.max && other.min <= self.max;
    }
    pub fn merge(&mut self, other: &Range) {
        if other.min < self.min {
            self.min = other.min
        }
        if other.max > self.max {
            self.max = other.max;
        }
    }
    pub fn value(&self) -> u64 {
        return self.max - self.min + 1;
    }
}

pub fn merge_interception(ranges: &mut Vec<Range>) -> bool {
    for current_position in 0..ranges.len() - 1 {
        let (left, right) = ranges.split_at_mut(current_position + 1);
        let current_range = &mut left[current_position];
        for peek in 0..right.len() {
            if current_range.intercepts(&right[peek]) {
                current_range.merge(&right[peek]);
                ranges.remove(current_position + 1 + peek);
                return true;
            }
        }
    }
    return false;
}


pub fn solve(input_path: &str) {
    let input = read_to_string(input_path)
        .expect("failed to read input");

    let mut ranges: Vec<Range> = vec![];
    for line in input.lines() {
        if line == "" { break; }
        if line.contains("-") {
            let bounds:Vec<&str> = line.split("-").collect();
            if bounds.len() != 2 { panic!("error parsing range"); }
            ranges.push(Range { min: bounds[0].parse().expect("error parsing min range"), max: bounds[1].parse().expect("error parsing max range") });
        }
    }

    ranges.sort_by(|range1, range2| {
        if range2.min > range1.min {
            return std::cmp::Ordering::Less;
        }
        return std::cmp::Ordering::Greater;
    });   

    loop { if !merge_interception(&mut ranges) { break; } }

    let solution:u64 = ranges.iter().map(|range| range.value()).sum();
    println!("Day 05 - Cafeteria\n  - Solution: {}\n", solution);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample() {
    }
}