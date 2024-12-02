use std::fs;

fn load_data() -> (Vec<i32>, Vec<i32>) {
    let result = fs::read_to_string("./inputs/day1.txt").unwrap();
    let result = result.split("\r\n").collect::<Vec<&str>>();
    let mut result_lists: (Vec<i32>, Vec<i32>) = (vec![], vec![]);
    for i in result {
        result_lists.0.push(i.split("   ").collect::<Vec<&str>>().get(0).unwrap().parse::<i32>().unwrap());
        result_lists.1.push(i.split("   ").collect::<Vec<&str>>().get(1).unwrap().parse::<i32>().unwrap());
    }
    return result_lists;
}

pub fn solve() -> i32 {
    let data = load_data();
    let mut list1 = data.0;
    let mut list2 = data.1;

    list1.sort();
    list2.sort();

    let mut result = 0;

    for i in 0..list1.len() {
        result += (list1[i] - list2[i]).abs();
    }

    return result;
}