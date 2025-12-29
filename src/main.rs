use std::env;

mod day01_secret_entrance;
mod day02_gift_shop;
mod day03_lobby;
mod day04_printing_department;
mod day05_cafeteria;

fn input_path(filename: &str) -> String {
    let home = env::home_dir().expect("failed to find homedir");
    let home = home.to_str().expect("failed to find homedir");
    format!("{}/Code/aoc2025/input/{}", home, filename)
}

fn main() {
    day01_secret_entrance::solve(input_path("day01.txt").as_str());
    day02_gift_shop::solve(input_path("day02.txt").as_str());
    day03_lobby::solve(input_path("day03.txt").as_str());
    day04_printing_department::solve(input_path("day04.txt").as_str());
    day05_cafeteria::solve(input_path("day05.txt").as_str());
}
