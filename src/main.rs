mod day1;
mod day2;

fn main() {
    println!("Advent of code 24:");
    println!("Day 1:");
    println!("{}", day1::solve());
    println!("{}", day1::solveSimilarity());
    println!("Day 2:");
    println!("{}", day2::solve());
    println!("{}", day2::solveTask2());
}
