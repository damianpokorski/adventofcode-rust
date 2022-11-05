use std::fs;

use itertools::Itertools;

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

fn counters_to_string(letters: &Vec<u32>) -> String {
    return letters
        .into_iter()
        .map(|counter| char::from_digit(counter + 10, 36).unwrap().to_string())
        .join("");
}

fn rule_1_three_letters_in_sequence(letters: &Vec<u32>) -> bool {
    for index in 2..letters.len() {
        let a = letters.get(index - 2).unwrap() + 2;
        let b = letters.get(index - 1).unwrap() + 1;
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

fn rule_3_contains_two_pairs(letters: &Vec<u32>) -> bool {
    let mut pairs_found: Vec<u32> = vec![];
    for index in 1..(&letters).len() {
        let previous_character = (&letters).get(index - 1).unwrap().clone();
        let current_character = (&letters).get(index).unwrap().clone();

        if previous_character == current_character && !pairs_found.contains(&current_character) {
            pairs_found.push(current_character.clone());
        }
    }
    return pairs_found.len() > 1;
}

fn optimize(letters: &mut Vec<u32>) -> bool {
    for index in 0..letters.len() {
        let mut clear_further = true;
        match letters[index] {
            8 => letters[index] = 9,
            11 => letters[index] = 12,
            14 => letters[index] = 15,
            _ => clear_further = false,
        }

        if clear_further {
            for inner_index in index + 1..letters.len() {
                letters[inner_index] = 0;
            }
            return true;
        }
    }
    return false;
}

fn increment(letters: &mut Vec<u32>) {
    for index in (0..letters.len()).rev() {
        if letters[index] == 25 {
            letters[index] = 0;
        } else {
            letters[index] = letters[index] + 1;
            break;
        }
    }
}

fn generate_new_password(password: String) -> String {
    let mut test = string_to_counters(password.to_string());

    println!("Start: {:?}", counters_to_string(&test));
    loop {
        {
            optimize(&mut test);
            increment(&mut test);
        }
        const BACKSPACE: char = 8u8 as char;

        print!("{}\rAfter: {:?}", BACKSPACE, counters_to_string(&test));
        if rule_1_three_letters_in_sequence(&test)
            && rule_2_no_i_l_o(&test)
            && rule_3_contains_two_pairs(&test)
        {
            println!("");
            println!("Next valid password: {:?}", counters_to_string(&test));
            return counters_to_string(&test);
        }
    }
}

pub fn puzzle() {
    let part1 = generate_new_password(read_file());
    println!("Part1: {:?}", part1);
    let part2 = generate_new_password(part1);
    println!("Part2: {:?}", part2);
}
