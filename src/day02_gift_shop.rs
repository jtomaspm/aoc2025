use std::fs::{read_to_string};

pub fn validate_id(id: &str) -> bool {
    if id.len() % 2 != 0 {
        return true;
    }
    let half = id.len() / 2;
    if id[..half-1] == id[half..] {
        return false;
    }
    return true;
}

pub fn validate_range(min: u64, max: u64) -> Vec<String> {
    let mut invalid_ids: Vec<String> = vec![];
    for id_num in min..max {
        let id = format!("{}",id_num);
        if !validate_id(id.as_str()) {
            invalid_ids.push(id);
        }
    }
    return invalid_ids;
}

pub fn solve(input_path: &str) {
    let mut invalid_ids: Vec<String> = vec![];
    read_to_string(input_path)
        .expect("failed to read input")
        .split(",")
        .for_each(|range| {
            let bounds:Vec<&str> = range.split("-").collect();
            if bounds.len() != 2 {
                panic!("id bounds has length: {}", bounds.len())
            }
            invalid_ids.append(
                &mut validate_range(
                    bounds[0].parse().expect(format!("invalid min range for ids: {}", bounds[0]).as_str()),
                    bounds[1].parse().expect(format!("invalid max range for ids: {}", bounds[1]).as_str()),
                )
            );
        });
    let mut solution:u64 = 0;
    for id in invalid_ids {
        solution += id.parse().expect("failed to parse invalid id");
    }
    println!("Day 02 - Gift Shop\n  - Solution: {}\n", "");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample() {
        assert_eq!(false, validate_id("55"));
        assert_eq!(false, validate_id("6464"));
        assert_eq!(false, validate_id("123123"));
        assert_eq!(true, validate_id("223123"));
    }
}