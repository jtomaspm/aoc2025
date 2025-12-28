use std::fs::{File, read_to_string};

#[derive(PartialEq)]
pub enum Direction {
    Right,
    Left
}
pub struct Dial {
    pub position: u32,
    pub range: u32,
}
impl Dial {
    pub fn rotate(&mut self, direction: Direction, amount: u32, count_target: u32) -> u32{
        let mut count = 0;
        let direction_mult:i32 = match direction {
            Direction::Left => -1,
            Direction::Right => 1
        };
        for _ in 0..amount {
            let next_position = self.position as i32 + direction_mult; 
            if next_position < 0 {
                self.position = self.range - 1;
            } else if next_position > (self.range - 1) as i32 {
                self.position = 0
            } else {
                self.position = next_position as u32;
            }
            if self.position == 0 {
                count += 1;
            }
        }
        return count;
    }
}
pub fn parse_line(line: &str) -> (Direction, u32) {
    let amount: u32 = line[1..].parse().expect("invalid amount in line");
    if line.starts_with("L") {
        return (Direction::Left, amount);
    }
    return (Direction::Right, amount);
}
pub fn solve(input_path: &str) {
    let mut dial = Dial{
        position: 50,
        range: 100,
    };
    let mut password = 0;
    read_to_string(input_path)
        .expect("failed to read input")
        .lines()
        .for_each(|line| {
            let (direction, amount) = parse_line(line);
            password += dial.rotate(direction, amount, 0);
        });
    println!("Day 01 - Secret Entrance\n  - Solution: {}\n", password);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dial() {
        let mut dial = Dial{
            position: 5,
            range: 100
        };
        dial.rotate(Direction::Left, 10, 0);
        assert!(dial.position==95);
        dial.rotate(Direction::Right, 5, 0);
        assert!(dial.position==0);
    }
    #[test]
    fn test_parse_line() {
        let line = "L5";
        let (direction, amount) = parse_line(line);
        assert!(direction==Direction::Left);
        assert!(amount==5);
        let line = "R10";
        let (direction, amount) = parse_line(line);
        assert!(direction==Direction::Right);
        assert!(amount==10);
    }
}