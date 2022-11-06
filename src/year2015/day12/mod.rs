use std::{collections::HashMap, fs};

use fancy_regex::Regex;
use itertools::Itertools;
use serde_json::{json, Value};

const PATH: &str = "src/year2015/day12/data.raw";

fn read_file() -> String {
    println!("Reading a file: {PATH}");
    return fs::read_to_string(PATH).expect("Should be able to read the file");
}

fn sum_of_numbers_in_strict(string: String) -> i32 {
    let regex = Regex::new(r"((-|)\d+)").unwrap();
    let captures = regex.captures_iter(string.as_str()).into_iter();
    let mut sum = 0;
    for capture in captures {
        let number: i32 = capture
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .to_string()
            .parse()
            .unwrap();
        sum = sum + number;
    }
    return sum;
}

fn part1() -> i32 {
    let result = read_file();
    return sum_of_numbers_in_strict(result);
}

fn recursive_value_counting(value: &Value) -> i64 {
    let red_value = json!("red");
    // It's an object
    if value.is_object() {
        // If we do not contain the red object - we containue diving in
        if !value.as_object().unwrap().values().contains(&red_value) {
            return value
                .as_object()
                .unwrap()
                .values()
                .map(|value| recursive_value_counting(value))
                .reduce(|a, b| a + b)
                .unwrap();
        }
    }
    if value.is_array() {
        return value
            .as_array()
            .unwrap()
            .into_iter()
            .map(|value| recursive_value_counting(value))
            .reduce(|a, b| a + b)
            .unwrap();
    }
    if value.is_number() {
        return value.as_i64().unwrap();
    }
    return 0;
}

fn part2() -> i64 {
    let result = read_file();

    // json parser
    let deserialized: HashMap<String, Value> = serde_json::from_str(&result).unwrap();

    let mut sum = 0;
    for (_index, (_raw, value)) in deserialized.into_iter().enumerate() {
        sum = sum + recursive_value_counting(&value);
    }
    return sum;
}

pub fn puzzle() {
    let part1 = part1();
    let part2 = part2();
    println!("Part1: {:?}", part1);
    println!("Part2: {:?}", part2);
}
