use std::vec;

use itertools::Itertools;

use crate::common::puzzle_data;

fn parse_input() -> Vec<(i32, i32, i32)> {
    return puzzle_data(std::file!())
        .split("\n")
        .into_iter()
        .map(|line| {
            // This is lazy :)
            let entry = line
                .to_string()
                .trim()
                .replace("    ", " ")
                .replace("   ", " ")
                .replace("  ", " ")
                .split(" ")
                .map(|x| x.to_string())
                .map(|x| x.parse::<i32>().unwrap_or(-1))
                .collect_vec();
            return (
                *(entry.get(0).unwrap_or(&-1)),
                *(entry.get(1).unwrap_or(&-1)),
                *(entry.get(2).unwrap_or(&-1)),
            );
        })
        .collect_vec();
}

fn valid_triangle(a: i32, b: i32, c: i32) -> bool {
    return a + b > c && b + c > a && c + a > b;
}

fn part1() -> usize {
    let mut counter = 0;

    for triangle in parse_input() {
        let (a, b, c) = triangle;
        if valid_triangle(a, b, c) {
            counter = counter + 1;
        }
    }

    return counter;
}

fn part2() -> i32 {
    // let mut row_a: Option<(i32, i32, i32)> = None;
    // let mut row_b: Option<(i32, i32, i32)> = None;
    // let mut row_c: Option<(i32, i32, i32)> = None;
    let mut counter: i32 = 0;

    let mut rows: Vec<(i32, i32, i32)> = vec![];

    for entry in parse_input() {
        // Build buffer
        rows.push(entry.clone());

        // Skip first & second insert
        if rows.len() < 3 {
            continue;
        }

        // Check and add relevant number after transposing in a dodgy manner
        counter = counter
            + vec![
                valid_triangle(
                    rows.get(0).unwrap().0,
                    rows.get(1).unwrap().0,
                    rows.get(2).unwrap().0,
                ),
                valid_triangle(
                    rows.get(0).unwrap().1,
                    rows.get(1).unwrap().1,
                    rows.get(2).unwrap().1,
                ),
                valid_triangle(
                    rows.get(0).unwrap().2,
                    rows.get(1).unwrap().2,
                    rows.get(2).unwrap().2,
                ),
            ]
            .into_iter()
            .map(|result| if result { 1 } else { 0 })
            .reduce(|a, b| a + b)
            .unwrap_or(0);
        rows.clear();
    }
    return counter;
}

pub fn puzzle() {
    let part1 = part1();
    let part2 = part2();
    println!("Part1: {:?}", part1);
    println!("Part2: {:?}", part2);
}
