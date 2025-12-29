use std::fs::read_to_string;

pub fn get_joltage(bank: &str) -> u64 {
    let mut digits:[u64;12] = [0,0,0,0,0,0,0,0,0,0,0,0];
    let bank_size = bank.len();
    for current in 0..bank_size {
        let left = bank_size - current;
        let end = digits.len();
        let mut start:i32 =  end as i32 - left as i32;
        if start < 0 {
            start = 0;
        }
        let start: usize = start as usize;
        for position in start..end {
            let digit = digits[position];
            let battery:u64 = bank[current..=current].parse().expect(format!("failed to get battery joltage: {}", &bank[current..=current]).as_str());
            if battery > digit {
                digits[position] = battery;
                if position < end - 1 {
                    for i in position+1..end {
                        digits[i] = 0;
                    }
                } 
                break;
            }
        }
    }

    let mut joltage = 0;
    for i in 0..12 {
        joltage += digits[i] * u64::pow(10, (digits.len() - i - 1) as u32)
    }
    return joltage;
}

pub fn solve(input_path: &str) {
    let mut solution:u64 = 0;
    read_to_string(input_path)
        .expect("failed to read input")
        .lines()
        .for_each(|line| {
            solution += get_joltage(line);
        });
    println!("Day 03 - Lobby\n  - Solution: {}\n", solution);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample() {
        let c1 = get_joltage("987654321111111");
        let c2 = get_joltage("811111111111119");
        let c3 = get_joltage("234234234234278");
        let c4 = get_joltage("818181911112111");
        let solution = c1 + c2 + c3 + c4;

        assert_eq!(987654321111, c1);
        assert_eq!(811111111119, c2);
        assert_eq!(434234234278, c3);
        assert_eq!(888911112111, c4);
        assert_eq!(3121910778619, solution);
    }
}