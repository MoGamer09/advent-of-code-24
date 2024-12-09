use std::fs;

fn load_data() -> Vec<Vec<char>> {
    let mut result = fs::read_to_string("./inputs/day8.txt").unwrap();
    let mut result = result.split("\r\n").collect::<Vec<&str>>();
    let mut result_lists: Vec<Vec<char>> = Vec::new();
    let mut i = 0;
    for result in result {
        result_lists.push(Vec::new());
        for j in 0..result.len() {
            result_lists[i].push(result.chars().nth(j).unwrap());
        }
        i += 1;
    }

    return result_lists;
}

pub fn solve() -> i32 {
    let data: Vec<Vec<char>> = load_data();

    let mut antinodes: Vec<(i32, i32)> = Vec::new();

    let mut result = 0;

    for y in 0..data.len() {
        for x in 0..data[0].len() {
            if(data[y][x] != '.'){
                for y2 in 0..data.len() {
                    for x2 in 0..data[0].len() {
                        if(y != y2 && x != x2 && data[y][x] == data[y2][x2]){
                            let antinode1Pos = (x as i32 + (x as i32 - x2 as i32), y as i32 + (y as i32 - y2 as i32));
                            let antinode2Pos = (x2 as i32 + (x2 as i32 - x as i32), y2 as i32 + (y2 as i32 - y as i32));
                            //println!("{}", data[y2][x2]);
                            if(inBounds(antinode1Pos.0, antinode1Pos.1) && !antinodes.contains(&antinode1Pos)) {
                                antinodes.push(antinode1Pos);
                            }

                            if(inBounds(antinode2Pos.0, antinode2Pos.1) && !antinodes.contains(&antinode2Pos)) {
                                antinodes.push(antinode2Pos);
                            }
                        }
                    }
                }
            }
        }
    }

    result = antinodes.len() as i32;
    return result;
}

fn inBounds(x : i32, y : i32) -> bool {
    let data = load_data();
    if(0 <= x && x < data.len() as i32 && 0 <= y && y < data[0].len() as i32) {
        return true;
    }
    return false;
}

pub(crate) fn solveTask2() -> i32 {
    let data: Vec<Vec<char>> = load_data();

    let mut antinodes: Vec<(i32, i32)> = Vec::new();

    let mut result = 0;

    for y in 0..data.len() {
        for x in 0..data[0].len() {
            if(data[y][x] != '.'){
                for y2 in 0..data.len() {
                    for x2 in 0..data[0].len() {
                        if(y != y2 && x != x2 && data[y][x] == data[y2][x2]){
                            for i in 0..100 {
                                let antinodePos = (x as i32 + i as i32 * (x as i32 - x2 as i32), y as i32 + i as i32 * (y as i32 - y2 as i32));
                                //println!("{}", data[y2][x2]);
                                if(inBounds(antinodePos.0, antinodePos.1) && !antinodes.contains(&antinodePos)) {
                                    antinodes.push(antinodePos);
                                }
                                else if(!inBounds(antinodePos.0, antinodePos.1)) {
                                    break;
                                }
                            }

                            for i in 0..-100 {
                                let antinodePos = (x as i32 + i as i32 * (x as i32 - x2 as i32), y as i32 + i as i32 * (y as i32 - y2 as i32));
                                //println!("{}", data[y2][x2]);
                                if(inBounds(antinodePos.0, antinodePos.1) && !antinodes.contains(&antinodePos)) {
                                    antinodes.push(antinodePos);
                                }
                                else if(!inBounds(antinodePos.0, antinodePos.1)) {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    result = antinodes.len() as i32;
    return result;
}