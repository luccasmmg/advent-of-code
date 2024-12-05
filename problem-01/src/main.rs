use std::fs;
use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::io::{self, BufRead};

//PART 1

//fn main() {
//    let file = File::open("input.txt").unwrap();
//    let reader = io::BufReader::new(file);
//
//    let mut first_list = Vec::new();
//    let mut second_list = Vec::new();
//
//    let mut distance = 0;
//    let mut length = 0;
//    for line in reader.lines() {
//        let line = line.unwrap();
//        let parts: Vec<&str> = line.split_whitespace().collect();
//        let first_part: i32 = parts.get(0).unwrap().parse().expect("Not a valid integer");
//        let second_part: i32 = parts.get(1).unwrap().parse().expect("Not a valid integer");
//        insert_sorted(&mut first_list, first_part);
//        insert_sorted(&mut second_list, second_part);
//        length += 1;
//    }
//    let mut iter_1 = first_list.iter();
//    let mut iter_2 = second_list.iter();
//    for i in 0..length {
//        let item_first_list = iter_1.next().unwrap();
//        let item_second_list  = iter_2.next().unwrap();
//        let distance_between_two_numbers = (item_first_list - item_second_list).abs();
//        distance += distance_between_two_numbers;
//    }
//    println!("Distance between two lists: {}", distance);
//}
//
//fn insert_sorted(vec: &mut Vec<i32>, value: i32) {
//    let pos = vec.binary_search(&value).unwrap_or_else(|e| e);
//    vec.insert(pos, value);
//}


fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut first_list = HashMap::new();
    let mut second_list = HashMap::new();

    let mut similarity_score = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let first_part: i32 = parts.get(0).unwrap().parse().expect("Not a valid integer");
        let second_part: i32 = parts.get(1).unwrap().parse().expect("Not a valid integer");
        match first_list.get_mut(&first_part) {
            Some(value) => {
                *value += 1;
            }
            None => {
                first_list.insert(first_part, 1);
            }
        }
        match second_list.get_mut(&second_part) {
            Some(value) => {
                *value += 1;
            }
            None => {
                second_list.insert(second_part, 1);
            }
        }
    }
    for (k, v) in first_list {
        match second_list.get(&k) {
            Some(value) => {
                similarity_score += (v * value) * k;
            }
            None => {}
        }
    }
    println!("Similarity score: {}", similarity_score);
}

