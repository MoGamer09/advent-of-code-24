use std::fs;

fn load_data() -> Vec<Vec<char>> {
    let mut result = fs::read_to_string("./inputs/day12.txt").unwrap();
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
    let mut data = load_data();
    let mut result= 0;

    for y in 0..data.len() {
        for x in 0..data[y].len() {
            if (data[y][x] != '.') {
                let target = data[y][x];
                let r = fill_area_recursive((x, y), &mut data, target);
                result += r.0 * r.1;

                for line in 0..data.len() {
                    for char in 0..data[line].len() {
                        if data[line][char] == '#' {
                            data[line][char] = '.';
                        }
                    }
                }
            }
        }
    }
    return result;
}

fn fill_area_recursive(pos: (usize, usize), data: &mut Vec<Vec<char>>, target: char) -> (i32, i32) {
    let dirs: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    data[pos.1][pos.0] = '#'; //Filler

    let mut result: (i32, i32) = (1, 0);
    for dir in dirs {
        let new_pos = (
            (pos.0 as i32 + dir.0) as usize,
            (pos.1 as i32 + dir.1) as usize,
        );
        if in_bounds(&data, new_pos) && (data[new_pos.1][new_pos.0] == target) {
            let r = fill_area_recursive(new_pos, data, target);
            result.0 += r.0;
            result.1 += r.1;
        }
        else if !in_bounds(&data, new_pos) || !(data[new_pos.1][new_pos.0] == '#' || data[new_pos.1][new_pos.0] == target) {
            result.1 += 1;
        }
    }
    return result;
}

pub fn in_bounds(map: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
    return pos.0 >= 0 && pos.0 < map.len() && pos.1 >= 0 && pos.1 < map[pos.0].len();
}
