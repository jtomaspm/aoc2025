use std::fs::{read_to_string};

pub fn validate_id(id: &str) -> bool {
    let half = id.len() / 2;
    for window_size in 1..=half {
        let window = &id[..window_size];
        let search_space = &id[window_size..];
        let mut start = 0;
        let mut valid = false;
        while start+window_size <= search_space.len() {
            let block = &search_space[start..start+window_size];
            if !(window == block) {
                valid = true;
                break;
            }
            start += window_size;
        }
        if !valid && start >= search_space.len() {
            return false;
        }
    }
    return true;
}

pub fn validate_range(min: u64, max: u64) -> Vec<String> {
    let mut invalid_ids: Vec<String> = vec![];
    for id_num in min..=max {
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
        let id:u64 = id.parse().expect("failed to parse invalid id");
        solution += id;
    }
    println!("Day 02 - Gift Shop\n  - Solution: {}\n", solution);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample() {
        assert_eq!(false, validate_id("55"));
        assert_eq!(false, validate_id("111"));
        assert_eq!(false, validate_id("6464"));
        assert_eq!(false, validate_id("123123"));
        assert_eq!(true, validate_id("223123"));

        assert_eq!(vec!["11", "22"], validate_range(11,22));
        assert_eq!(vec!["99", "111"], validate_range(95,115));
        assert_eq!(vec!["1188511885"], validate_range(1188511880,1188511890));
        assert_eq!(vec!["222222"], validate_range(222220,222224));
        assert_eq!(Vec::<String>::new(), validate_range(1698522,1698528));
        assert_eq!(vec!["446446"], validate_range(446443,446449));
        assert_eq!(vec!["38593859"], validate_range(38593856,38593862));
        assert_eq!(vec!["565656"], validate_range(565653,565659));
        assert_eq!(vec!["824824824"], validate_range(824824821,824824827));
        assert_eq!(vec!["2121212121"], validate_range(2121212118,2121212124));
    }
}