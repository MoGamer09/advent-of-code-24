use std::fs;

fn load_data() -> Vec<Vec<char>> {
    let mut result = fs::read_to_string("./inputs/day4.txt").unwrap();
    let mut result = result.split("\n").collect::<Vec<&str>>();
    let mut result_lists: Vec<Vec<char>> = Vec::new();
    let validChars = ['X', 'M', 'A', 'S'];
    for i in 0..result.len() {
        result_lists.push(Vec::new());
        for j in result[i].chars() {
            if (validChars.contains(&j)) {
                result_lists[i].push(j);
            }
        }
    }

    return result_lists;
}

pub fn solve() -> i32 {
    let data = load_data();
    let searchword = ['X', 'M', 'A', 'S'];
    let mut result = 0;

    let dirs: [(i32, i32); 8] = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    for line in 0..data.len() {
        for column in 0..data[line].len() {
            for dir in dirs {
                let mut x = column;
                let mut y = line;

                'inner: for currentStep in 0..searchword.len() {
                    if (y < 0 || y >= data.len() || x < 0 || x >= data[y].len()) {
                        break 'inner;
                    }

                    if (data[y][x] != searchword[currentStep]) {
                        break 'inner;
                    }

                    if currentStep == searchword.len() - 1 {
                        result += 1;
                    }

                    x = ((x as i32) + dir.0) as usize;
                    y = ((y as i32) + dir.1) as usize;
                }
            }
        }
    }

    return result;
}

pub fn solveTask2() -> i32 {
    let data = load_data();
    let searchwords = [
        ['A', 'S', 'M', 'M', 'S'],
        ['A', 'M', 'M', 'S', 'S'],
        ['A', 'M', 'S', 'S', 'M'],
        ['A', 'S', 'S', 'M', 'M'],
    ];
    let mut result = 0;

    let positionOffsets: [(i32, i32); 5] = [(0, 0), (1, 1), (-2, 0), (0, -2), (2, 0)];

    for line in 0..data.len() {
        for column in 0..data[line].len() {
            let mut x = column;
            let mut y = line;
            let mut stillWorks: [bool ; 4] = [true, true, true, true];

            for currentStep in 0..positionOffsets.len() {
                x = ((x as i32) + positionOffsets[currentStep].0) as usize;
                y = ((y as i32) + positionOffsets[currentStep].1) as usize;

                if (y < 0 || y >= data.len() || x < 0 || x >= data[y].len()) {
                    break;
                }

                for i in 0..searchwords.len() {
                    if (data[y][x] != searchwords[i][currentStep]) {
                        stillWorks[i] = false;
                    }
                }

                if(!stillWorks.contains(&true))
                {
                    break;
                }

                if currentStep == searchwords[0].len() - 1 {
                    /*println!(
                        "{} . {} \n . {} . \n {} . {} \n \n",
                        data[y][x - 2],
                        data[y][x],
                        data[y + 1][x - 1],
                        data[y + 2][x - 2],
                        data[y + 2][x],
                    );*/
                    result += 1;
                }
            }
        }
    }

    return result;
}

//?
//463

//?
//384
//380
