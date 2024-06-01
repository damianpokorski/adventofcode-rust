use std::thread;

use crate::common::puzzle_data;
use itertools::Itertools;
use md5;

fn part1() -> String {
    let contents: String = puzzle_data(std::file!());

    let buffer: String = contents.clone();
    let mut addition = 0;
    let mut answer: Vec<String> = vec![];

    loop {
        // Generate test string, generate md5 checksum, convert it to string
        let test = [&buffer, &addition.to_string()].into_iter().join("");
        let digest = md5::compute(test.clone().as_bytes());
        let digest_str = format!("{:x}", digest);

        // Check if digest starts with 5 zeroes
        let is_match = digest_str.clone().chars().take(5).all(|x| x == '0');
        if is_match {
            let hint = digest_str.clone().chars().take(6).skip(5).join("");
            answer.push(hint.clone());

            println!("[Part 1] {:?}/8 - {:?}", answer.len(), answer.join(""));
        }

        // Incrementing counter, only finish once we have all 8 characters
        addition = addition + 1;
        if answer.len() == 8 {
            return answer.join("");
        }
    }
}

fn part2() -> String {
    let contents: String = puzzle_data(std::file!());

    let buffer: String = contents.clone();
    let mut addition = 0;
    let mut answer: Vec<String> = vec![
        "_".to_string(),
        "_".to_string(),
        "_".to_string(),
        "_".to_string(),
        "_".to_string(),
        "_".to_string(),
        "_".to_string(),
        "_".to_string(),
    ];

    let mut answer_progress = 0;
    let mut final_answer: String = "".to_string();
    loop {
        // Generate test string, generate md5 checksum, convert it to string
        let test = [&buffer, &addition.to_string()].into_iter().join("");
        let digest = md5::compute(test.clone().as_bytes());
        let digest_str = format!("{:x}", digest);

        // Check if digest starts with 5 zeroes
        let is_match = digest_str.clone().chars().take(5).all(|x| x == '0');
        if is_match {
            let hint = digest_str.clone().chars().take(7).skip(6).join("");
            let index = digest_str
                .clone()
                .chars()
                .take(6)
                .skip(5)
                .join("")
                .parse::<usize>()
                .unwrap_or(10);

            if index < 8 {
                if answer.get(index).unwrap_or(&"_".to_string()) == &"_".to_string() {
                    // Insert at index, remove pushed entry
                    answer.insert(index, hint);
                    answer.remove(index + 1);

                    final_answer = answer.clone().into_iter().join("").to_string();

                    answer_progress = answer
                        .clone()
                        .into_iter()
                        .filter(|code| code != "_")
                        .count();

                    println!("[Part 2] {:?}/8 - {:?}", answer_progress, final_answer);
                }
            }
        }

        // Incrementing counter, only finish once we have all 8 characters
        addition = addition + 1;
        if answer_progress == 8 {
            return final_answer;
        }
    }
}

pub fn puzzle() {
    let part_1_result = thread::spawn(|| part1());
    let part_2_result = thread::spawn(|| part2());

    let part1_result = part_1_result.join().unwrap_or("".to_string());
    let part2_result = part_2_result.join().unwrap_or("".to_string());
    println!("-------");
    println!("Part 1: {:?}", part1_result);
    println!("Part 2: {:?}", part2_result);
}
