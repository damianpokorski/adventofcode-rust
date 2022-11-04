use std::fs;

use itertools::Itertools;
use fancy_regex::Regex;

const PATH: &str = "src/year2015/day11/data.raw";

fn read_file() -> String {
    println!("Reading a file: {PATH}");
    return fs::read_to_string(PATH).expect("Should be able to read the file");
}

fn string_to_counters(string: String) -> Vec<u32> {
    return string
        .to_ascii_lowercase()
        .chars()
        .into_iter()
        .map(|character| character.to_digit(36).unwrap() - 10)
        .collect();
}

fn counters_to_string(letters: Vec<u32>) -> String {
    return letters
        .into_iter()
        .map(|counter| char::from_digit(counter+10, 36).unwrap().to_string())
        .join("");
}

fn rule_1_three_letters_in_sequence(letters: &Vec<u32>) -> bool {
    for index in 2..letters.len() {
        let a = letters.get( index - 2).unwrap() + 2;
        let b = letters.get( index - 1).unwrap() + 1;
        let c = letters.get(index).unwrap().clone();
        if a == b && b == c {
            return true;
        }
    }
    
    return false;
}

fn rule_2_no_i_l_o(letters: &Vec<u32>) -> bool {
    for index in 0..letters.len() {
        let letter = letters.get(index).unwrap().clone();
        if letter == 8 || letter == 11 || letter == 14 {
            return false;
        }
    }
    
    return true;
}

fn rule_3_contains_two_pairs(letters: &Vec<u32>) -> bool{
    let mut previous_character = 0;
    let mut pairs_found:Vec<u32> = vec![];
    for index in 0..(&letters).len() {
        let current_character = (&letters).get(index).unwrap().clone();

        if previous_character == current_character && !pairs_found.contains(&current_character){
            pairs_found.push(current_character.clone())
        }

        // Save current characters
        previous_character = current_character.clone();
    }
    return pairs_found.len() > 1;
}

fn increment(letters: Vec<u32>) {

}

fn part1() -> usize {
    let result = read_file();
    let alphabet = String::from("abcdefghijklmnopqrstuvwxyz");
    println!("{:?}", string_to_counters(alphabet.clone()));
    println!("{:?}", counters_to_string(string_to_counters(alphabet.clone())));
    println!("{:?}", rule_1_three_letters_in_sequence(&string_to_counters(alphabet.clone())));
    println!("{:?}", rule_2_no_i_l_o(&string_to_counters("a".to_string())));
    println!("{:?}", rule_3_contains_two_pairs(&string_to_counters("aaaa".to_string())));
    println!("{:?}", rule_3_contains_two_pairs(&string_to_counters("aabb".to_string())));

    println!("{:?}", string_to_counters("iol".to_string()));
    return result.lines().count();
}

fn part2() -> usize {
    return 0;
}

pub fn puzzle() {
    let result = part1();
    println!("Part1: {:?}", result);
    let result = part2();
    println!("Part2: {:?}", result);
}
