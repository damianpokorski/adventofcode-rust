use itertools::Itertools;

use crate::common::puzzle_data;

fn part1() -> i32 {
    let contents = puzzle_data(std::file!());

    return contents
        .split("\n\n")
        .into_iter()
        .map(|segment| {
            segment
                .split("\n")
                .into_iter()
                .map(|number| number.parse::<i32>().unwrap())
                .reduce(|a, b| a.clone() + b.clone())
                .unwrap()
        })
        .collect_vec()
        .into_iter()
        .reduce(|a, b| if a > b { a } else { b })
        .unwrap();
}

fn part2() -> i32 {
    let contents = puzzle_data(std::file!());

    return contents
        .split("\n\n")
        .into_iter()
        .map(|segment| {
            segment
                .split("\n")
                .into_iter()
                .map(|number| number.parse::<i32>().unwrap())
                .reduce(|a, b| a.clone() + b.clone())
                .unwrap()
        })
        .collect_vec()
        .into_iter()
        .sorted_by(|a, b| b.cmp(a))
        .collect_vec()[0..3]
        .into_iter()
        .map(|a| a.clone())
        .reduce(|a, b| a + b)
        .unwrap();
}

pub fn puzzle() {
    let part1 = part1();
    let part2 = part2();
    println!("Part1: {:?}", part1);
    println!("Part2: {:?}", part2);
}
