use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);
    let mut _safe = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let safe = check_levels(numbers.clone());
        if safe {
            _safe += 1;
        } else {
            println!("Not safe at first");
            let mut numbers_minus_one: Vec<Vec<i32>> = [].to_vec();
            for i in 0..(numbers.len()) {
                let mut new_numbers = numbers.clone();
                new_numbers.remove(i);
                numbers_minus_one.push(new_numbers);
            }
            let safe_again = numbers_minus_one.iter().any(|x| check_levels(x.clone()));
            if safe_again {
                println!("Safe again");
                _safe += 1;
            } else {
                println!("Line is: {:?}", line);
            }
        }
    }
    println!("Safe: {}", _safe);
}

fn check_levels(numbers: Vec<i32>) -> bool {
    println!("Checking: {:?}", numbers);
    let decreasing = numbers[0] < numbers[1];
    let mut safe = true;
    let mut index = 0;
    for i in numbers.windows(2) {
        index += 1;
        if !safe {
            continue;
        }
        let distance_between_two_numbers = (i[0] - i[1]).abs();
        if (i[0] - i[1]) < 0 && !decreasing {
            safe = false;
            continue;
        }
        if (i[0] - i[1]) > 0 && decreasing {
            safe = false;
            continue;
        }
        if (i[0] == i[1]) {
            safe = false;
            continue;
        }
        if distance_between_two_numbers > 3 {
            safe = false;
            continue;
        }
    }
    println!("Safe: {}", safe);
    return safe;
}
