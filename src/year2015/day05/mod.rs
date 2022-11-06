use std::{fs, vec};

use fancy_regex::Regex;

const PATH: &str = "src/year2015/day05/data.raw";

fn read_file() -> String {
    println!("Reading a file: {PATH}");
    return fs::read_to_string(PATH).expect("Should be able to read the file");
}

fn part1() -> (i32, i32) {
    let mut naughty = 0;
    let mut nice = 0;
    // Look ups
    let vowels = vec!["a", "e", "i", "o", "u"];
    let disallowed_strings: Vec<String> = (vec!["ab", "cd", "pq", "xy"])
        .into_iter()
        .map(|x| String::from(x))
        .collect();
    let empty_letter = "_";

    // Iterate through lines
    for line in read_file()
        .split_ascii_whitespace()
        .map(|x| String::from(x))
        .into_iter()
    {
        // Prepare per character counters & flags
        let letters = line.split("").into_iter();
        let mut vowel_counter = 0;
        let mut repeated_letters = 0;
        let mut last_letter = empty_letter;
        let mut disallowed = false;

        // Iterate through characters
        for letter in letters {
            // Vowel counting
            if vowels.contains(&letter) {
                vowel_counter += 1;
            }
            // Repeated letters
            if last_letter == letter {
                repeated_letters += 1;
            }
            // Disallowed
            let last_two = format!("{}{}", last_letter, letter);
            if disallowed == false && disallowed_strings.contains(&last_two) {
                disallowed = true;
                break;
            }

            last_letter = letter;
        }
        // Decides
        if disallowed == false && vowel_counter >= 3 && repeated_letters >= 1 {
            nice += 1;
        } else {
            naughty += 1;
        }
    }
    return (naughty, nice);
}

fn part2() -> (i32, i32) {
    let mut naughty = 0;
    let mut nice = 0;

    // Iterate through lines
    for line in read_file()
        .split_ascii_whitespace()
        .map(|x| String::from(x))
        .into_iter()
    {
        let single_letter_repeat_with_gap = Regex::new(r"([a-z]{1}).\1")
            .unwrap()
            .is_match(&line)
            .unwrap();
        let two_letter_reapting = Regex::new(r"([a-z]{2}).*?\1")
            .unwrap()
            .is_match(&line)
            .unwrap();

        if single_letter_repeat_with_gap && two_letter_reapting {
            nice += 1;
        } else {
            naughty += 1;
        }
    }

    return (naughty, nice);
}

pub fn puzzle() {
    let part1 = part1();
    let part2 = part2();
    println!("Part1: {:?}", part1);
    println!("Part2: {:?}", part2);
}
