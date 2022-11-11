use itertools::Itertools;

use crate::common::puzzle_data;

#[derive(Debug)]
struct Sue {
    id: i32,
    children: i32,
    cats: i32,
    samoyeds: i32,
    pomeranians: i32,
    akitas: i32,
    vizslas: i32,
    goldfish: i32,
    trees: i32,
    cars: i32,
    perfumes: i32,
}

fn ingest() -> Vec<Sue> {
    return puzzle_data(std::file!())
        .split("\n")
        .into_iter()
        .map(|string| string.replace("Sue ", ""))
        .map(|string| {
            let mut pieces = string
                .split(", ")
                .map(|sub| sub.to_string())
                .into_iter()
                .collect_vec();

            let sue_id = (&string).split_once(":").unwrap().0;

            pieces[0] = pieces[0]
                .replacen(sue_id, ":", 1)
                .split_once(":")
                .into_iter()
                .last()
                .unwrap()
                .1
                .to_string()
                .split_at(2)
                .1
                .to_string();

            let mut template = Sue {
                id: str::parse(sue_id).unwrap(),
                children: -1,
                cats: -1,
                samoyeds: -1,
                pomeranians: -1,
                akitas: -1,
                vizslas: -1,
                goldfish: -1,
                trees: -1,
                cars: -1,
                perfumes: -1,
            };

            for piece in pieces {
                let piece = piece.split_once(": ").unwrap();

                let left = piece.0;
                let right = str::parse::<i32>(piece.1).unwrap();

                match left {
                    "children" => template.children = right,
                    "cats" => template.cats = right,
                    "samoyeds" => template.samoyeds = right,
                    "pomeranians" => template.pomeranians = right,
                    "akitas" => template.akitas = right,
                    "vizslas" => template.vizslas = right,
                    "goldfish" => template.goldfish = right,
                    "trees" => template.trees = right,
                    "cars" => template.cars = right,
                    "perfumes" => template.perfumes = right,
                    _ => (),
                }
            }

            return template;
        })
        .collect_vec();
}

fn part1() -> i32 {
    let known_aunt = Sue {
        id: -1,
        children: 3,
        cats: 7,
        samoyeds: 2,
        pomeranians: 3,
        akitas: 0,
        vizslas: 0,
        goldfish: 5,
        trees: 3,
        cars: 2,
        perfumes: 1,
    };

    let mut potential_aunts: Vec<Sue> = vec![];
    for potentil_aunt in ingest() {
        if ((known_aunt.children == potentil_aunt.children || potentil_aunt.children == -1)
            && (known_aunt.cats == potentil_aunt.cats || potentil_aunt.cats == -1)
            && (known_aunt.samoyeds == potentil_aunt.samoyeds || potentil_aunt.samoyeds == -1)
            && (known_aunt.pomeranians == potentil_aunt.pomeranians
                || potentil_aunt.pomeranians == -1)
            && (known_aunt.akitas == potentil_aunt.akitas || potentil_aunt.akitas == -1)
            && (known_aunt.vizslas == potentil_aunt.vizslas || potentil_aunt.vizslas == -1)
            && (known_aunt.goldfish == potentil_aunt.goldfish || potentil_aunt.goldfish == -1)
            && (known_aunt.trees == potentil_aunt.trees || potentil_aunt.trees == -1)
            && (known_aunt.cars == potentil_aunt.cars || potentil_aunt.cars == -1)
            && (known_aunt.perfumes == potentil_aunt.perfumes || potentil_aunt.perfumes == -1))
        {
            potential_aunts.push(potentil_aunt);
        }
    }

    return potential_aunts.first().unwrap().id;
}

fn part2() -> i32 {
    let known_aunt = Sue {
        id: -1,
        children: 3,
        cats: 7,
        samoyeds: 2,
        pomeranians: 3,
        akitas: 0,
        vizslas: 0,
        goldfish: 5,
        trees: 3,
        cars: 2,
        perfumes: 1,
    };

    let mut potential_aunts: Vec<Sue> = vec![];
    for potentil_aunt in ingest() {
        if ((known_aunt.children == potentil_aunt.children || potentil_aunt.children == -1)
            && (known_aunt.cats < potentil_aunt.cats || potentil_aunt.cats == -1)
            && (known_aunt.samoyeds == potentil_aunt.samoyeds || potentil_aunt.samoyeds == -1)
            && (known_aunt.pomeranians > potentil_aunt.pomeranians
                || potentil_aunt.pomeranians == -1)
            && (known_aunt.akitas == potentil_aunt.akitas || potentil_aunt.akitas == -1)
            && (known_aunt.vizslas == potentil_aunt.vizslas || potentil_aunt.vizslas == -1)
            && (known_aunt.goldfish > potentil_aunt.goldfish || potentil_aunt.goldfish == -1)
            && (known_aunt.trees < potentil_aunt.trees || potentil_aunt.trees == -1)
            && (known_aunt.cars == potentil_aunt.cars || potentil_aunt.cars == -1)
            && (known_aunt.perfumes == potentil_aunt.perfumes || potentil_aunt.perfumes == -1))
        {
            potential_aunts.push(potentil_aunt);
        }
    }

    return potential_aunts.first().unwrap().id;
}

pub fn puzzle() {
    let part1 = part1();
    let part2 = part2();
    println!("Part1: {:?}", part1);
    println!("Part2: {:?}", part2);
}
