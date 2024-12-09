use std::ffi::CString;
use std::fs;

fn load_data() -> Vec<(i64, Vec<i64>)> {
    let mut result = fs::read_to_string("./inputs/day7.txt").unwrap();
    let mut result = result.split("\r\n").collect::<Vec<&str>>();
    let mut result_lists: Vec<(i64, Vec<i64>)> = Vec::new();
    for i in 0..result.len() {
        //println!("{}", &result[i]);
        let mut line = (
            result[i].split(": ").collect::<Vec<&str>>()[0]
                .parse()
                .unwrap(),
            Vec::new(),
        );
        for j in result[i].split(": ").collect::<Vec<&str>>()[1]
            .split(" ")
            .collect::<Vec<&str>>()
        {
            line.1.push(j.parse::<i64>().unwrap());
        }
        result_lists.push(line);
    }

    return result_lists;
}

pub(crate) fn solve() -> i64 {
    let mut result = 0;
    let mut data = load_data();

    let mut base: i64 = 2;
    for line in data {
        for i in 0..base.pow(line.1.len() as u32) {
            let mut lineResult = 0;
            for numberIndex in 0..line.1.len() {
                if (i >> numberIndex) & 1 == 0 {
                    lineResult *= line.1[numberIndex];
                } else {
                    lineResult += line.1[numberIndex];
                }

                if (lineResult > line.0) {
                    break;
                }
            }

            if (lineResult == line.0) {
                result += line.0;
                //println!("Found result: {} = {}", lineResult, line.0);
                break;
            }
        }
    }

    return result;
}

//2314935962622

pub(crate) fn solveTask2() -> i64 {
    let mut result = 0;
    let mut data = load_data();

    let mut base: i64 = 3;
    for line in data {
        let mut terCounter: String = "00000000000000000000000000000000".to_string();
        for i in 0..base.pow(line.1.len() as u32) {
            let mut lineResult: i128 = 0;
            for numberIndex in 0..line.1.len() {
                let mut currentNumber: i128 = line.1[numberIndex]  as i128;
                if terCounter.chars().nth(numberIndex).unwrap() == '2' {
                    lineResult = (lineResult.to_string()
                        + &*line.1[numberIndex].to_string())
                        .parse::<i128>()
                        .unwrap();
                }
                if (terCounter.chars().nth(numberIndex).unwrap() == '0') {
                    lineResult *= currentNumber as i128;
                } else if (terCounter.chars().nth(numberIndex).unwrap() == '1') {
                    lineResult += currentNumber  as i128;
                }
                if (lineResult > line.0  as i128) {
                    break;
                }
            }

            if (lineResult == line.0  as i128) {
                result += line.0;
                print!("Found result: {} = ", line.0);
                for numberIndex in 0..line.1.len() {
                    if terCounter.chars().nth(numberIndex).unwrap() == '0' {
                        print!(" * ")
                    }
                    if terCounter.chars().nth(numberIndex).unwrap() == '1' {
                        print!(" + ")
                    }
                    if terCounter.chars().nth(numberIndex).unwrap() == '2' {
                        print!(" || ")
                    }
                    print!("{}", line.1[numberIndex]);
                }
                print!("\n");
                break;
            }

            let mut index = 0;

            while terCounter.chars().nth(index).unwrap() == '2' {
                terCounter = replace_nth_char(&terCounter, index, '0');
                index += 1;
            }
            if (terCounter.chars().nth(index).unwrap() == '0') {
                terCounter = replace_nth_char(&terCounter, index, '1');
            } else if (terCounter.chars().nth(index).unwrap() == '1') {
                terCounter = replace_nth_char(&terCounter, index, '2');
            }
        }
    }

    return result;
}

fn replace_nth_char(s: &str, idx: usize, newchar: char) -> String {
    s.chars()
        .enumerate()
        .map(|(i, c)| if i == idx { newchar } else { c })
        .collect()
}

//?
//2509178774189
//2315186321421

