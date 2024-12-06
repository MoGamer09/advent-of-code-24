use std::collections::HashMap;
use std::fs;

fn load_data() -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut result = fs::read_to_string("./inputs/day5.txt").unwrap();
    let mut result = result.split("\r\n\r\n").collect::<Vec<&str>>();
    let mut result_dictionary: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in result[0].split("\r\n").collect::<Vec<&str>>() {
        let key = rule.split("|").collect::<Vec<&str>>()[0]
            .parse::<i32>()
            .unwrap();
        let value = rule.split("|").collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap();
        if !result_dictionary.contains_key(&key) {
            result_dictionary.insert(key, Vec::new());
        }
        result_dictionary
            .entry(key.clone())
            .or_insert_with(Vec::new)
            .push(value);
    }

    let mut result_lists: Vec<Vec<i32>> = Vec::new();

    let mut i = 0;
    for manual in result[1].split("\r\n").collect::<Vec<&str>>() {
        result_lists.push(Vec::new());
        for page in manual.split(",").collect::<Vec<&str>>() {
            result_lists[i].push(page.parse::<i32>().unwrap());
        }
        i += 1;
    }

    return (result_dictionary, result_lists);
}

pub fn solve() -> i32 {
    let data = load_data();
    let lists = data.1;
    let rules = data.0;

    let mut result = 0;

    let mut i = -1;
    'lists: for list in lists {
        i += 1;
        let mut alreadyPrintedPages: Vec<i32> = Vec::new();
        for pageIndex in 0..list.len() {
            let page = list[pageIndex];
            for rule in rules.get(&page).unwrap() {
                if (alreadyPrintedPages.contains(rule)) {
                    //println!("Dont do this!");
                    continue 'lists;
                }
            }
            alreadyPrintedPages.push(page);
        }
        //println!("{} in {}", ((list.len() / 2)) as usize, list.len());
        result += list[(list.len() / 2)];
    }
    return result;
}

pub fn solveTask2() -> i32 {
    let data = load_data();
    let lists = data.1;
    let rules = data.0;

    let mut result = 0;

    let mut i = -1;
    for mut list in lists {
        i += 1;

        if(!isCorrectList(&list, &rules)) {
            while !isCorrectList(&list, &rules) {
                let mut alreadyPrintedPages: Vec<i32> = Vec::new();
                'outer: for pageIndex in 0..list.len() {
                    let page = list[pageIndex];
                    for rule in rules.get(&page).unwrap() {
                        if (alreadyPrintedPages.contains(rule)) {
                            //Broken

                            for printedPageIndex in 0..alreadyPrintedPages.len() {
                                if (alreadyPrintedPages[printedPageIndex] == *rule) {
                                    list.remove(pageIndex);
                                    list.insert(printedPageIndex, page);
                                    list.remove(printedPageIndex + 1);
                                    list.insert(pageIndex, alreadyPrintedPages[printedPageIndex]);
                                    break;
                                }
                            }

                            //println!("{:?}", list);
                            break 'outer;
                        }
                    }
                    alreadyPrintedPages.push(page);
                }
            }

            result += list[(list.len() / 2)];
            //println!("{}", list[(list.len() / 2)]);
        }
    }
    return result;
}

fn isCorrectList(list: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> bool {
    let mut alreadyPrintedPages: Vec<i32> = Vec::new();
    for pageIndex in 0..list.len() {
        let page = list[pageIndex];
        for rule in rules.get(&page).unwrap() {
            if (alreadyPrintedPages.contains(rule)) {
                //Broken
                return false;
            }
        }
        alreadyPrintedPages.push(page);
    }
    return true;
}

//13135
//12352
//12291
//?
