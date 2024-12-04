mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    println!("Advent of code 24:");
    println!("Day 1:");
    println!("{}", day1::solve());
    println!("{}", day1::solveSimilarity());
    println!("Day 2:");
    println!("{}", day2::solve());
    println!("{}", day2::solveTask2());
    println!("Day 3:");
    println!("{}", day3::solve());
    println!("{}", day3::solveTask2());
    println!("Day 4:");
    println!("{}", day4::solve());
    println!("{}", day4::solveTask2());
}
