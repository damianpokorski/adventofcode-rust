use std::collections::HashMap;

use itertools::Itertools;

use crate::common::puzzle_data;

#[derive(Debug)]
struct Entry {
    raw: String,
    sector_id: i32,
    valid: bool,
    // Used for debugging
    #[allow(dead_code)]
    expected_checksum: String,
    #[allow(dead_code)]
    calculated_checksum: String,
}

fn process(line: &str) -> Entry {
    // Remove unecessary biots & split into two
    let pieces = line.replace("]", "");
    let pieces = pieces.split('[').collect_vec();

    // Split characters into two groups
    let raw_characters = pieces.get(0).unwrap_or(&"").chars();
    let checksum = pieces.get(1).unwrap_or(&"").chars().join("");

    // Split characters and digits
    let raw_characters = raw_characters.clone().join(&"");
    let character_chunks = raw_characters.split('-').collect_vec();
    let digits = character_chunks.last().unwrap_or(&"").chars().join(&"");

    // Extract string without the number
    let characters = raw_characters[0..raw_characters.len() - digits.len() - 1].chars();

    // Convert digits to int
    let digits = digits.parse::<i32>().unwrap_or(0);

    // Convert Chars to vector for user use
    let characters = characters.collect_vec();

    // Chars calculate
    let counters = (&characters)
        .into_iter()
        .filter(|x| *x != &'-')
        .counts_by(|value| value);

    let groups = counters
        .clone()
        .into_iter()
        .into_group_map_by(|value| value.1)
        .into_iter()
        .map(|(key, value)| {
            let sorted_value = value
                .into_iter()
                .map(|entry| entry.0.clone())
                .sorted()
                .collect_vec();
            return (key, sorted_value);
        })
        .collect::<HashMap<usize, Vec<char>>>();

    let expected_checksum = (&groups)
        .into_iter()
        .sorted_by(|a, b| b.0.cmp(a.0))
        .map(|a| a.1.into_iter().join(""))
        .join("")
        .chars()
        .into_iter()
        .take(5)
        .join("");

    let is_valid = checksum == expected_checksum;

    return Entry {
        raw: characters.into_iter().join(""),
        sector_id: digits,
        expected_checksum: checksum,
        calculated_checksum: expected_checksum,
        valid: is_valid,
    };
}

fn part1() -> i32 {
    return puzzle_data(std::file!())
        .lines()
        .into_iter()
        .map(|line| process(line))
        .filter(|line| {
            // Debug log
            // println!(
            //     " - {:?} - {:?} == {:?} = {:?}",
            //     line.raw, line.expected_checksum, line.calculated_checksum, line.valid
            // );

            return line.valid;
        })
        .map(|line| line.sector_id)
        .reduce(|a, b| a + b)
        .unwrap_or(0);
}

fn part2() -> i32 {
    return puzzle_data(std::file!())
        .lines()
        .into_iter()
        .map(|line| process(line))
        .filter(|line| {
            let ascii_start = 97;
            let alphabet_length = 26;
            let offset: u8 = (line.sector_id % alphabet_length as i32) as u8;
            let decoded_line = line
                .raw
                .chars()
                .into_iter()
                .map(|char| {
                    if char == '-' {
                        return char;
                    }
                    let character_base = char.to_ascii_lowercase() as u8 - ascii_start;
                    let character_converted = ((character_base + offset) % alphabet_length) as u8;
                    let final_ascii = ascii_start + character_converted;
                    return final_ascii as char;
                })
                .join("");

            // The decoded names are really fun actually :)
            // println!("{:?} - {:?}", decoded_line, line.sectorId);

            return decoded_line == "northpole-object-storage";
        })
        .map(|a| a.sector_id)
        .reduce(|a, b| a + b)
        .unwrap_or(0);
}

pub fn puzzle() {
    let part1 = part1();
    println!("Part1: {:?}", part1);
    let part2 = part2();
    println!("Part2: {:?}", part2);
}
