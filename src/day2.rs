use std::fs;

fn load_data() -> Vec<Vec<i32>> {
    let mut result = fs::read_to_string("./inputs/day2.txt").unwrap();
    let mut result = result.split("\r\n").collect::<Vec<&str>>();
    let mut result_lists: Vec<Vec<i32>> = Vec::new();
    for i in 0..result.len() {
        result_lists.push(Vec::new());
        for j in result[i].split(" ").collect::<Vec<&str>>() {
            result_lists[i].push(j.parse().unwrap());
        }
    }
    return result_lists;
}

pub(crate) fn solve() -> i32 {
    let data = load_data();
    let mut result = 0;
    for seq in data {
        let mut lastLevel = -1;
        let mut sign = 0;
        let mut stable = true;
        for level in seq {
            if (lastLevel == -1) {
                lastLevel = level;
                continue;
            }
            if (level - lastLevel).abs() <= 3 && (level - lastLevel).abs() != 0 {
                let new_sign = if level - lastLevel > 0 { 1 } else { -1 };
                if (sign == 0) {
                    sign = new_sign;
                    lastLevel = level;
                    continue;
                }
                if (new_sign != sign) {
                    stable = false;
                    break;
                }
                lastLevel = level;
            } else {
                stable = false;
                break;
            }
        }
        if (stable) {
            result += 1;
        }
    }
    return result;
}

pub(crate) fn solveTask2() -> i32 {
    let data = load_data();
    let mut result = 0;
    for seq in data {
        for removedLevel in 0..seq.len() {
            let mut last_level = -1;
            let mut sign = 0;
            let mut stable = true;
            for level in 0..seq.len() {
                if(removedLevel == level) {
                    continue;
                }
                if (last_level == -1) {
                    last_level = seq[level];
                    continue;
                }
                if (seq[level] - last_level).abs() <= 3 && (seq[level] - last_level).abs() != 0 {
                    let new_sign = if seq[level] - last_level > 0 { 1 } else { -1 };
                    if (sign == 0) {
                        sign = new_sign;
                        last_level = seq[level];
                        continue;
                    }
                    if (new_sign != sign) {
                        stable = false;
                        break;
                    }
                    last_level = seq[level];
                } else {
                    stable = false;
                    break;
                }
            }
            if (stable) {
                result += 1;
                break;
            }
        }
    }
    return result;
}
