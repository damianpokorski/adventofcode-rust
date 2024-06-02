use itertools::Itertools;

use crate::common::puzzle_data;

fn part1() -> String {
    let contents = puzzle_data(std::file!());

    let mut most_frequent: Vec<String> = vec![];
    for index in 0..8 {
        // Iterate through all strings -
        let chars = contents
            .split("\n")
            .into_iter()
            .map(|line| line.get(index..index + 1).unwrap_or(&"_"))
            .collect_vec();

        let sorted_by_frequency = chars
            .clone()
            .into_iter()
            .unique()
            .map(|char| {
                return (
                    char,
                    (chars.clone())
                        .into_iter()
                        .filter(|&other_char| other_char.eq(char))
                        .count(),
                );
            })
            .sorted_by(|&a, &b| b.1.cmp(&a.1))
            .map(|tuple| tuple.0)
            .collect_vec();

        let most_frequent_char = sorted_by_frequency.first().unwrap();

        // Save most frequent
        most_frequent.push(most_frequent_char.to_string());
    }
    return most_frequent.join(&"");
}

fn part2() -> usize {
    let contents = puzzle_data(std::file!());
    return contents.lines().count();
}

pub fn puzzle() {
    let part1 = part1();
    let part2 = part2();
    println!("Part1: {:?}", part1);
    println!("Part2: {:?}", part2);
}
