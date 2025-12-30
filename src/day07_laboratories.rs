use std::{fs::read_to_string};

pub struct Node {
    value: Beam,
    parent: Option<Box<Node>>,
    children: Vec<Box<Node>>
}

pub struct PathTree {
    root: Box<Node>
} 

#[derive(PartialEq)]
pub struct Move {
    from: Beam,
    to: Beam
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Beam {
    row: usize,
    col: usize
}
impl Beam {
    pub fn should_split(&self, map: &Vec<Vec<char>>) -> bool {
        return map[self.row][self.col] == '^';
    }
    fn can_move(&self, map: &Vec<Vec<char>>) -> bool {
        return self.row <= map.len() - 2  
    }
    pub fn move_down(&mut self, map: &Vec<Vec<char>>) -> Option<Move> {
        if self.can_move(map) {
            let from = Beam { row: self.row, col: self.col };
            self.row += 1;
            let to = Beam { row: self.row, col: self.col };
            return Some(Move{ from, to });
        }
        return None;
    }
    pub fn split(&self) -> (Vec<Beam>, Vec<Move>) {
        let from = Beam{row: self.row, col: self.col};
        let to_l = Beam{row: self.row, col: self.col - 1};
        let to_r = Beam{row: self.row, col: self.col + 1};
        return (vec![to_l, to_r], vec![Move{from: from, to: to_l}, Move{from: from, to: to_r}]);
    }
}

pub fn find_start(map: &Vec<Vec<char>>) -> Beam {
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == 'S' {
                return Beam { row, col };
            }
        }
    }
    panic!("start not found");
}

pub fn simulation(map: &Vec<Vec<char>>) -> u64 {
    let start = find_start(map);
    let mut beams: Vec<Beam> = vec![start];
    let mut hits:u64 = 0;
    let mut moves: Vec<Move> = vec![]; 

    loop {
        let mut frame_moves = 0;

        let mut to_remove: Vec<usize> = vec![]; 
        let mut new_beams: Vec<Beam> = vec![];
        for beam_id in 0..beams.len() {
            let beam = &mut beams[beam_id];
            if let Some(m) = beam.move_down(&map) {
                frame_moves += 1;
                if !moves.contains(&m) { moves.push(m); }
                if beam.should_split(map) {
                    let (mut nb, nm) = beam.split();
                    to_remove.push(beam_id);
                    new_beams.append(&mut nb);
                    hits += 1;
                    for nm_e in nm {
                        if !moves.contains(&nm_e) { moves.push(nm_e); }
                    }
                }
            }
        }
        to_remove.sort_by(|b1, b2| {
            if b1 > b2 {
                return std::cmp::Ordering::Less;
            }
            return std::cmp::Ordering::Greater;
        });
        for beam_id in to_remove {
            beams.remove(beam_id);
        }
        for beam in new_beams {
            if !beams.iter().any(|e| e.col == beam.col && e.row == beam.row) {
                beams.push(beam);
            }
        }
        if frame_moves == 0 {
            break;
        }
    }
    let mut root: Box<Node>;
    for m in moves {
        if m.from == start {
            root = Box::new(Node { 
                value: m.from, 
                parent: None, 
                children: vec![] 
            });
            let child = Box::new(Node { 
                value: m.from, 
                parent: None, 
                children: vec![] 
            });
            break;
        }
    }

    return hits;
}

pub fn solve(input_path: &str) {
    let map: Vec<Vec<char>> = read_to_string(input_path)
        .expect("failed to read input")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    
    let solution:u64 = simulation(&map);
    println!("Day 07 - Laboratories\n  - Solution: {}\n", solution);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample() {
    }
}