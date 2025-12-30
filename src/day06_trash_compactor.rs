use std::fs::read_to_string;

pub enum Operation {
    Sum,
    Multiplication
}

pub struct Expression {
    pub values: Vec<u64>,
    pub operation: Operation
} 

impl Expression {
    pub fn new(data_col: &Vec<String>) -> Expression {
        let mut values: Vec<u64> = vec![];
        for i in 0..data_col.len()-1 {
            values.push(data_col[i].parse().expect("invalid data value"));
        }
        let sign = data_col.last().expect("no sign in data");
        return Expression { values, operation: match sign.as_str() {
            "+" => Operation::Sum,
            "*" => Operation::Multiplication,
            invalid => panic!("invalid sign in data: {}", invalid)
        } };
    }
    pub fn solve(&self) -> u64 {
        match self.operation {
            Operation::Sum => self.values.iter().sum(),
            Operation::Multiplication => self.values.iter().copied().reduce(|e1, e2| e1 * e2).expect("failed to multiply values"),
        }
    }
}

pub fn solve(input_path: &str) {
    let mut data:Vec<Vec<String>> = vec![];
    read_to_string(input_path)
        .expect("failed to read input")
        .lines()
        .for_each(|line| {
            let mut token = "".to_string(); 
            let mut tokens: Vec<String> = vec![];
            line.chars().for_each(|c| {
                if c == ' ' {
                    if token != "" {
                        tokens.push(token.clone());
                        token = "".to_string();
                    }
                }else{
                    token.push(c);
                }
            });
            if token != "" {
                tokens.push(token.clone());
                token = "".to_string();
            }
            data.push(tokens);
        });
    let data = data;
    
    let mut expressions: Vec<Expression> = vec![];
    for col_index in 0..data[0].len() {
        let mut col:Vec<String> = vec![];
        for row_index in 0..data.len() {
            col.push(data[row_index][col_index].clone());
        }
        expressions.push(Expression::new(&col));
    }

    let solution:u64 = expressions.iter().map(|e| e.solve()).sum();
    
    println!("Day 06 - Trash Compactor (Part 1)\n  - Solution: {}\n", solution);
}

pub fn solve_part2(input_path: &str) {
    let input: Vec<Vec<char>> = read_to_string(input_path)
        .expect("failed to read input")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    
    let mut expressions: Vec<Expression> = vec![]; 
    let mut last_sign = ' ';
    let mut raw_expression: Vec<String> = vec![];
    let mut number = "".to_string();
    for col in 0..input[0].len() {
        number.clear();
        for row in 0..input.len() - 1 {
            if input[row][col] != ' ' {
                number.push(input[row][col]);
            }
        }
        let sign_row = input.last().expect("invalid input lenght")[col];
        if sign_row != ' ' {
            last_sign = sign_row;
        }
        if number == "" {
            raw_expression.push(last_sign.to_string());
            expressions.push(Expression::new(&raw_expression));
            raw_expression.clear();
        } else {
            raw_expression.push(number.clone());
        }
    }
    if raw_expression.len() != 0 {
        raw_expression.push(last_sign.to_string());
        expressions.push(Expression::new(&raw_expression));
    }

    let solution:u64 = expressions.iter().map(|e| e.solve()).sum();
    println!("Day 06 - Trash Compactor (Part 2)\n  - Solution: {}\n", solution);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample() {
        let c1_data = vec!["123".to_string(), "45".to_string(), "6".to_string(), "*".to_string()];
        let c1 = Expression::new(&c1_data).solve();
        assert_eq!(33210, c1);

        let c1_data = vec!["328".to_string(), "64".to_string(), "98".to_string(), "+".to_string()];
        let c1 = Expression::new(&c1_data).solve();
        assert_eq!(490, c1);

        let c1_data = vec!["51".to_string(), "387".to_string(), "215".to_string(), "*".to_string()];
        let c1 = Expression::new(&c1_data).solve();
        assert_eq!(4243455, c1);

        let c1_data = vec!["64".to_string(), "23".to_string(), "314".to_string(), "+".to_string()];
        let c1 = Expression::new(&c1_data).solve();
        assert_eq!(401, c1);

    }
}