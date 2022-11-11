use itertools::Itertools;

use crate::common::puzzle_data;

fn part1() -> (i32, usize) {
    let target_container_size = 150;

    let containers = puzzle_data(std::file!())
        .split_ascii_whitespace()
        .into_iter()
        .map(|row| str::parse::<i32>(row).unwrap())
        .into_iter()
        .sorted()
        .collect_vec();

    let mut valid_combinations = 0;
    let mut smallest_combination: Vec<i32> = containers.clone();

    for containers_to_try in 1..containers.len() {
        for container_combination in (0..containers.len())
            .combinations(containers_to_try)
            .filter(|combination| {
                for i in 1..combination.len() {
                    if combination.get(i).unwrap() < combination.get(i - 1).unwrap() {
                        return false;
                    }
                }
                return true;
            })
            .into_iter()
        {
            let combination = container_combination
                .into_iter()
                .map(|container_index| containers.get(container_index).unwrap().clone())
                .collect_vec();

            let combination_size = (&combination)
                .into_iter()
                .map(|value| value.clone())
                .reduce(|a, b| a + b)
                .unwrap();

            if combination_size == target_container_size {
                valid_combinations = valid_combinations + 1;
                if combination.len() < smallest_combination.len() {
                    smallest_combination = combination.clone();
                }
            }
        }
    }

    return (valid_combinations, smallest_combination.len());
}

fn part2(containers_to_try: usize) -> usize {
    println!("Trying for X containers - {:?}", containers_to_try);
    let target_container_size = 150;

    let containers = puzzle_data(std::file!())
        .split_ascii_whitespace()
        .into_iter()
        .map(|row| str::parse::<i32>(row).unwrap())
        .into_iter()
        .sorted()
        .collect_vec();

    let mut valid_combinations = 0;

    //
    for container_combination in (0..containers.len())
        .combinations(containers_to_try)
        .filter(|combination| {
            for i in 1..combination.len() {
                if combination.get(i).unwrap() < combination.get(i - 1).unwrap() {
                    return false;
                }
            }
            return true;
        })
        .into_iter()
    {
        let combination = container_combination
            .into_iter()
            .map(|container_index| containers.get(container_index).unwrap().clone())
            .collect_vec();

        let combination_size = (&combination)
            .into_iter()
            .map(|value| value.clone())
            .reduce(|a, b| a + b)
            .unwrap();

        if combination_size == target_container_size {
            valid_combinations = valid_combinations + 1;
        }
    }

    return valid_combinations;
}

pub fn puzzle() {
    let part1 = part1();
    let part2 = part2(part1.1);
    println!("Part1: {:?}", part1.0);
    println!("Part2: {:?}", part2);
}
