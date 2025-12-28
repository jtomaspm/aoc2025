mod day01_secret_entrance;
mod day02_gift_shop;

fn input_path(filename: &str) -> String {
    format!("/home/pop/Code/aoc2025/input/{}", filename)
}

fn main() {
    day01_secret_entrance::solve(input_path("day01.txt").as_str());
    day02_gift_shop::solve(input_path("day02.txt").as_str());
}
