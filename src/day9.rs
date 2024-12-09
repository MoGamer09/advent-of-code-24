use std::fs;
use std::process::id;
use std::thread::current;

fn load_data() -> Vec<Option<i32>> {
    let mut result = fs::read_to_string("./inputs/day9.txt").unwrap();
    let mut result_list: Vec<Option<i32>> = Vec::new();
    for i in 0..result.len() {
        for par in 0..result.chars().nth(i).unwrap().to_digit(10).unwrap() {
            if (i % 2 == 0) {
                result_list.push(Some((i / 2) as i32));
            } else {
                result_list.push(None);
            }
        }
    }
    return result_list;
}

pub(crate) fn solve() -> i128 {
    let mut data = load_data();
    let mut result: i128 = 0;

    'outer: for tileIndex in 0..data.len() {
        if (data[tileIndex] == None) {
            for otherTileIndex in (0..data.len()).rev() {
                if (otherTileIndex <= tileIndex) {
                    //Only the end is empty; so done
                    break 'outer;
                }
                if (data[otherTileIndex] != None) {
                    let id = data[otherTileIndex].unwrap();
                    data[otherTileIndex] = None;
                    data[tileIndex] = Some(id);
                    continue 'outer;
                }
            }
        }
    }

    for idIndex in 0..data.len() {
        if (data[idIndex] == None) {
            break;
        }
        result += idIndex as i128 * data[idIndex].unwrap() as i128;
    }

    return result;
}

pub fn solveTask2() -> i128 {
    let mut data = load_data();
    let mut result: i128 = 0;

    let mut backIndex = data.len();
    let mut backFileLength = 0;
    let mut currentId = -1;

    let mut frontIndex = 0;
    let mut frontSpaceLength = 0;

    while backIndex > 1 {

        backIndex -= 1;

        if (data[backIndex] != None) {
            currentId = data[backIndex].unwrap();
            backFileLength += 1;
        }

        if (data[backIndex] != None && data[backIndex] != data[backIndex - 1]) {
            //println!("Checking {}", currentId);
            //So end of individual file reached
            while frontIndex < backIndex {
                if (data[frontIndex] == None) {
                    frontSpaceLength += 1;
                } else {
                    frontSpaceLength = 0;
                }

                if (frontSpaceLength == backFileLength) {
                    //Found a large enough gap
                    for i in (frontIndex - (frontSpaceLength - 1))..(frontIndex + 1) {
                        data[i] = Some(currentId);
                    }
                    for i in (backIndex)..(backIndex + backFileLength) {
                        data[i] = None;
                    }
                    break;
                }

                frontIndex += 1;
            }
            frontIndex = 0;
            backFileLength = 0;
            currentId = -1;
            frontSpaceLength = 0;
        }
    }

    for idIndex in 0..data.len() {
        if (data[idIndex] != None) {
            result += idIndex as i128 * data[idIndex].unwrap() as i128;
        }
    }

    return result;
}

fn printData(data: &Vec<Option<i32>>) {
    for idIndex in 0..data.len() {
        match data[idIndex] {
            Some(id) => {
                print!("{}", id)
            }
            None => {
                print!(".")
            }
        }
    }
    print!("\n");
}
