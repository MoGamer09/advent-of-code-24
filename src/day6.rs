use std::collections::HashMap;
use std::fs;
use std::thread::current;

fn load_data() -> Vec<Vec<char>> {
    let mut result = fs::read_to_string("./inputs/day6.txt").unwrap();
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

pub(crate) fn solve() -> i32 {
    let dirs: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    let mut currentDir = 0;

    let mut result = 0;
    let mut data = load_data();
    //println!("{:?}", data);
    let mut y: i32 = 0;
    let mut x: i32 = 0;

    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if (data[i][j] == '^') {
                y = i as i32;
                x = j as i32;
            }
        }
    }

    while x >= 0 && y >= 0 && (y as usize) < data.len() && (x as usize) < data[0].len() {
        if (data[y as usize][x as usize] != 'X') {
            result += 1;
            data[y as usize][x as usize] = 'X';
            //println!("X,Y: {},{}", x, y);
        }

        if (y + dirs[currentDir % 4].1 < 0
            || y + dirs[currentDir % 4].1 >= data.len() as i32
            || x + dirs[currentDir % 4].0 >= data[0].len() as i32
            || x + dirs[currentDir % 4].0 < 0)
        {
            break;
        }

        if (data[(y + dirs[currentDir % 4].1) as usize][(x + dirs[currentDir % 4].0) as usize]
            == '#')
        {
            currentDir += 1;
            continue;
        }

        y = (y + dirs[currentDir % 4].1);
        x = (x + dirs[currentDir % 4].0);
    }
    return result;
}

//?
//5128

pub(crate) fn solveTask2() -> i32 {
    let mut result = 0;

    let dirs: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    let mut currentDir = 0;

    let mut data = load_data();
    //println!("{:?}", data);
    let mut y: i32 = 0;
    let mut x: i32 = 0;

    let mut loopPositions: Vec<(i32, i32)> = Vec::new();

    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if (data[i][j] == '^') {
                y = i as i32;
                x = j as i32;
            }
        }
    }

    while x >= 0 && y >= 0 && (y as usize) < data.len() && (x as usize) < data[0].len() {
        let mut isIn = false;
        for loopPositionIndex in 0..loopPositions.len() {
            if(loopPositions[loopPositionIndex].0 == x && loopPositions[loopPositionIndex].1 == y) {
                isIn = true;
            }
        }

        if(!isIn) {
            let tile = data[y as usize][x as usize];
            data[y as usize][x as usize] = '#';
            if (guardInLoop(&data)) {
                loopPositions.push((x as i32, y as i32));
                //println!("loop position: {},{}", x,y);
            }
            data[y as usize][x as usize] = tile;
        }

        if (y + dirs[currentDir % 4].1 < 0
            || y + dirs[currentDir % 4].1 >= data.len() as i32
            || x + dirs[currentDir % 4].0 >= data[0].len() as i32
            || x + dirs[currentDir % 4].0 < 0)
        {
            break;
        }

        if (data[(y + dirs[currentDir % 4].1) as usize][(x + dirs[currentDir % 4].0) as usize]
            == '#')
        {
            currentDir += 1;
            continue;
        }

        y = (y + dirs[currentDir % 4].1);
        x = (x + dirs[currentDir % 4].0);
    }

    return loopPositions.len() as i32;
}

fn guardInLoop(game: &Vec<Vec<char>>) -> bool {
    let dirs: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    let mut currentDir = 0;

    //println!("{:?}", data);
    let mut y: i32 = 0;
    let mut x: i32 = 0;

    for i in 0..game.len() {
        for j in 0..game[i].len() {
            if (game[i][j] == '^') {
                y = i as i32;
                x = j as i32;
            }
        }
    }

    let mut turn_poses: Vec<(i32, i32, i32)> = Vec::new(); //x,y,currentdir%4

    while x >= 0 && y >= 0 && (y as usize) < game.len() && (x as usize) < game[0].len() {
        if (y + dirs[currentDir % 4].1 < 0
            || y + dirs[currentDir % 4].1 >= game.len() as i32
            || x + dirs[currentDir % 4].0 >= game[0].len() as i32
            || x + dirs[currentDir % 4].0 < 0)
        {
            break;
        }

        if (game[(y + dirs[currentDir % 4].1) as usize][(x + dirs[currentDir % 4].0) as usize]
            == '#')
        {
            currentDir += 1;
            for turnPos in &turn_poses {
                if(turnPos.0 == x && turnPos.1 == y && turnPos.2 == (currentDir % 4) as i32) {
                    return true;
                }
            }
            turn_poses.push((x, y, (currentDir % 4) as i32));
            continue;
        }

        y = (y + dirs[currentDir % 4].1);
        x = (x + dirs[currentDir % 4].0);
    }
    return false;
}

//2402
//?
