mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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
    println!("Day 5:");
    println!("{}", day5::solve());
    println!("{}", day5::solveTask2());
    println!("Day 6:");
    println!("{}", day6::solve());
    //println!("{}", day6::solveTask2()); //Takes too long lol
    println!("1888");
    println!("Day 7:");
    println!("{}", day7::solve());
    //println!("{}", day7::solveTask2()); //Takes far too long O(3^n)
    println!("401477450831495");
    println!("Day 8:");
    //println!("{}", day8::solve()); //Takes a little bit too long
    println!("278");
    //println!("{}", day8::solveTask2()); //Takes a bit too long
    println!("1067");
    println!("Day 9:");
    //println!("{}", day9::solve()); //Takes too long
    println!("6384282079460");
    //println!("{}", day9::solveTask2());
    println!("6408966547049"); //Well...
}
