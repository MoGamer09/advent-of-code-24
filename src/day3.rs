use regex::Regex;
use std::fs;

fn load_data() -> String {
    let result = fs::read_to_string("./inputs/day3.txt").unwrap();
    return result;
}

pub fn solve() -> i32 {
    let data = load_data();
    let rx = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    //println!("{}", rx.replace_all(&data, "").to_string());
    let mut result = 0;
    for cap in rx.captures_iter(&data) {
        let a = cap[1].parse::<i32>().unwrap();
        let b = cap[2].parse::<i32>().unwrap();
        //println!("{} * {}", a, b);
        result += a * b;
    }
    return result;
}

pub fn solveTask2() -> i32 {
    let mut data = "mul(2,2)don't()don't()don't()mul(1,2)do()do()mdo()ul(2,3)mul(5,4)don't()mul(8,1)";
    let data = load_data();
    let rx = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let dorx = Regex::new(r"don't\(\).*?do\(\)").unwrap();
    let newData = dorx.replace_all(&data, "").to_string();

    let mut result = 0;
    for cap in rx.captures_iter(&newData) {
        let a = cap[1].parse::<i32>().unwrap();
        let b = cap[2].parse::<i32>().unwrap();
        //println!("{} * {}", a, b);
        result += a * b;
    }
    return result;
}

//191183308
//107752201

//106506233 ?

//?
