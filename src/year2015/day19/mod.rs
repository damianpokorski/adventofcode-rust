use itertools::Itertools;

use crate::common::puzzle_data;

fn ingest() -> (Vec<(String, String)>, Vec<String>) {
    let contents = puzzle_data(std::file!());

    let (swaps, molecules) = (&contents).split_once("\n\n").unwrap();

    let swaps_tuples = swaps
        .split("\n")
        .into_iter()
        .map(|swap| swap.split_once(" => ").unwrap())
        .map(|tuple| (tuple.0.to_string(), tuple.1.to_string()))
        .collect_vec();

    let mut molecules_chain: Vec<String> = vec![];
    let mut buffer = "".to_string();

    for character in (&molecules).chars().map(|char| char.to_string()) {
        if buffer.len() > 0 && character.chars().last().unwrap().is_uppercase() {
            molecules_chain.push(buffer.clone());
            buffer = "".to_string();
        }
        // Add current character to buffer
        buffer.push_str(character.as_str());
    }
    if buffer.len() > 0 {
        molecules_chain.push(buffer);
    }

    return (swaps_tuples.clone(), molecules_chain.clone());
}

fn part1() -> usize {
    let (swaps, molecules) = ingest();
    println!("Swaps");
    for swap in (&swaps).into_iter() {
        println!("{0} => {1}", swap.0, swap.1);
    }
    println!("");
    println!("Molecules: {0}", (&molecules).into_iter().join(" => "));

    let mut combinations: Vec<String> = vec![];

    // Iterate through original chain
    for (molecule_index, molecule) in (&molecules).into_iter().enumerate() {
        let left = (&molecules)[0..molecule_index].join("");
        let right = (&molecules)[molecule_index + 1..].join("");

        // println!("{0}_{1}", left, right);

        // Find out potential replacements
        for replacement in (&swaps)
            .into_iter()
            .filter(|swap| swap.0.clone() == *molecule)
        {
            // println!("Replacements for {0} = {1}", molecule, replacement.1);

            // Add mutateted entry
            let mutated_entry = vec![left.clone(), replacement.1.clone(), right.clone()].join("");
            if !combinations.contains(&mutated_entry) {
                combinations.push(mutated_entry);
            }
        }
    }

    return combinations.into_iter().unique().count();
}

fn part2() -> usize {
    return 0;
}

pub fn puzzle() {
    let part1 = part1();
    let part2 = part2();
    println!("Part1: {:?}", part1);
    println!("Part2: {:?}", part2);
}
