use std::collections::HashMap;
use std::fs;

fn load_data() -> Vec<i128> {
    let mut result = fs::read_to_string("./inputs/day11.txt").unwrap();
    let mut result_list: Vec<i128> = Vec::new();
    let result = result.split(" ").collect::<Vec<&str>>();
    for i in result {
        result_list.push(i.parse::<i128>().unwrap());
    }
    return result_list;
}

pub fn solve() -> i32 {
    let mut data = load_data();
    for i in 0..25 {
        data = step(data);
    }
    let mut result = data.len() as i32;
    //println!("{:?}", data);
    return result;
}

fn step(data: Vec<i128>) -> Vec<i128> {
    let mut result: Vec<i128> = Vec::new();
    for i in 0..data.len() {
        if (data[i] == 0) {
            result.push(1);
        } else if (data[i].to_string().chars().count() % 2 == 0) {
            let split_index = data[i].to_string().chars().count() / 2;
            result.push(
                data[i]
                    .to_string()
                    .split_at(split_index)
                    .0
                    .to_string()
                    .parse::<i128>()
                    .unwrap(),
            );
            result.push(
                data[i]
                    .to_string()
                    .split_at(split_index)
                    .1
                    .to_string()
                    .parse::<i128>()
                    .unwrap(),
            );
        } else {
            result.push(data[i] * 2024);
        }
    }
    return result;
}

pub fn solveTask2() -> i128 {
    let mut data = load_data();
    let mut lookup = HashMap::new();
    let mut result : i128 = 0;
    for number in data {
        //println!("{}:", number);
        let mut single_result = 0;
        for i in 0..76 {
            let r = get_number_of_added_stones_for_number(i, number, &lookup);
            //println!("{}: {}", i, r.0);
            lookup = r.1;
            single_result = r.0;
        }
        result += single_result;
    }
    //println!("{:?}", data);
    return result;
}

pub fn get_number_of_added_stones_for_number(
    steps: i32,
    mut number: i128,
    oldlookups: &HashMap<i128, Vec<i128>>,
) -> (i128, HashMap<i128, Vec<i128>>) {
    //println!("evaluating: {} for steps: {}, lookups {:?}", number, steps,oldlookups);

    if (steps == 0) {
        return (1, HashMap::from([(number, [1].to_vec())]));
    }

    let mut result = 0;
    let mut lookups : HashMap<i128, Vec<i128>> = oldlookups.clone();

    for newNumber in step([number].to_vec()) {
        if (lookups.contains_key(&newNumber) && lookups[&newNumber].len() >= (steps) as usize) {
            //println!("looked up number: {}, for step: {} and found: {}", newNumber, steps-1, lookups[&newNumber][(steps - 1) as usize]);
            result += lookups[&newNumber][(steps - 1) as usize];
        } else {
            let r = get_number_of_added_stones_for_number(steps - 1, newNumber, &lookups);
            result += r.0;
            for map in r.1 {
                lookups.insert(map.0, map.1);
            }
        }
    }

    if(lookups.contains_key(&number) && lookups[&number].len() == (steps) as usize) {
        //println!("Wrote {}, at step {} ({}), for number {}", result, steps, lookups[&number].len() - 1, number);
        lookups.get_mut(&number).expect("REASON").push(result);
    }
    else {
        //println!("Lookup write failed for number: {}", number);
    }

    //println!("lookups: {:?}", lookups);
    return (result, lookups);
}
