use itertools::Itertools;

use crate::common::puzzle_data;

fn solution(ascending: bool) -> String {
    let contents = puzzle_data(std::file!());

    let mut most_frequent: Vec<String> = vec![];
    for index in 0..8 {
        // Iterate through all strings -grab nth
        let chars = contents
            .split("\n")
            .into_iter()
            .map(|line| line.get(index..index + 1).unwrap_or(&"_"))
            .collect_vec();

        let char_counters = chars.clone().into_iter().counts_by(|char| char);

        // Order characters by frequency
        // Part 1 = Most common
        // Part 2 = Least common
        let sorted_by_frequency = char_counters
            .into_iter()
            .sorted_by(|&a, &b| {
                return match ascending {
                    true => a.1.cmp(&b.1),
                    false => b.1.cmp(&a.1),
                };
            })
            .map(|tuple| tuple.0)
            .collect_vec();

        // Grab first
        let most_frequent_char = sorted_by_frequency.first().unwrap();

        // Save most frequent
        most_frequent.push(most_frequent_char.to_string());
    }
    return most_frequent.join(&"");
}

pub fn puzzle() {
    let part1 = solution(false);
    let part2 = solution(true);
    println!("Part1: {:?}", part1);
    println!("Part2: {:?}", part2);
}
