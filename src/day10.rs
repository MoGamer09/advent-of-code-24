use std::fs;

fn load_data() -> Vec<Vec<i32>> {
    let mut result = fs::read_to_string("./inputs/day10.txt").unwrap();
    let mut result = result.split("\r\n").collect::<Vec<&str>>();
    let mut result_lists: Vec<Vec<i32>> = Vec::new();
    let mut i = 0;
    for result in result {
        result_lists.push(Vec::new());
        for j in 0..result.len() {
            result_lists[i].push(result.chars().nth(j).unwrap().to_digit(10).unwrap() as i32);
        }
        i += 1;
    }

    return result_lists;
}

pub fn solve() -> i32 {
    let data = load_data();
    let mut result : i32 = 0;

    for y in 0..data.len() {
        for x in 0..data[0].len() {
            if (data[y][x] == 0) {
                result += find_paths_recursive(&data, (x as i32, y as i32), true).len() as i32;
            }
        }
    }

    return result;
}

pub fn find_paths_recursive(map: &Vec<Vec<i32>>, pos: (i32, i32), distinct: bool) -> Vec<(i32, i32)> {
    let dirs: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    if (map[pos.1 as usize][pos.0 as usize] == 9) {
        return [pos].to_vec();
    }

    let mut foundPaths: Vec<(i32, i32)> = Vec::new();

    for dir in dirs {
        if (in_bounds(&map, (pos.0 + dir.0, pos.1 + dir.1))
            && map[(pos.1 + dir.1) as usize][(pos.0 + dir.0) as usize]
                == map[pos.1 as usize][pos.0 as usize] + 1)
        {
            for newPos in find_paths_recursive(&map, (pos.0 + dir.0, pos.1 + dir.1), distinct) {
                if (!distinct || !foundPaths.contains(&newPos)) {
                    foundPaths.push(newPos);
                }
            }
        }
    }

    return foundPaths;
}

pub fn in_bounds(map: &Vec<Vec<i32>>, pos: (i32, i32)) -> bool {
    return pos.0 >= 0
        && pos.0 < map.len() as i32
        && pos.1 >= 0
        && pos.1 < map[pos.0 as usize].len() as i32;
}

pub fn solveTask2() -> i32 {
    let data = load_data();
    let mut result : i32 = 0;

    for y in 0..data.len() {
        for x in 0..data[0].len() {
            if (data[y][x] == 0) {
                result += find_paths_recursive(&data, (x as i32, y as i32), false).len() as i32;
            }
        }
    }

    return result;
}