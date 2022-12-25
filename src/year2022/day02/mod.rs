use itertools::Itertools;

use crate::common::puzzle_data;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug)]
struct Round {
    my_move: Move,
    other_move: Move,
    move_score: i32,
    win: bool,
    draw: bool,
    score: i32,
}

fn ingest(contents: String) -> Vec<Round> {
    return contents
        .split('\n')
        .into_iter()
        .map(|row| (row.get(0..1).unwrap(), row.get(2..).unwrap()))
        .map(|pair| {
            return (
                match pair.1 {
                    "X" => Move::Rock,
                    "Y" => Move::Paper,
                    "Z" => Move::Scissor,
                    _ => Move::Scissor,
                },
                match pair.0 {
                    "A" => Move::Rock,
                    "B" => Move::Paper,
                    "C" => Move::Scissor,
                    _ => Move::Scissor,
                },
                match pair.1 {
                    "X" => 1,
                    "Y" => 2,
                    "Z" => 3,
                    _ => 0,
                },
            );
        })
        .map(|pair| {
            let mut round = Round {
                my_move: pair.0,
                other_move: pair.1,
                score: 0,
                win: false,
                draw: false,
                move_score: pair.2,
            };
            round.win = (round.my_move == Move::Rock && round.other_move == Move::Scissor)
                || (round.my_move == Move::Scissor && round.other_move == Move::Paper)
                || (round.my_move == Move::Paper && round.other_move == Move::Rock);

            round.draw = round.my_move == round.other_move;

            round.score = round.move_score
                + if round.win {
                    6
                } else if round.draw {
                    3
                } else {
                    0
                };

            return round;
        })
        .collect_vec();
}

fn part1() -> i32 {
    let contents = puzzle_data(std::file!());
    for round in ingest(contents.clone()) {
        println!("{:?}", round);
    }
    return ingest(contents.clone())
        .into_iter()
        .map(|round| round.score)
        .reduce(|a, b| a + b)
        .unwrap();
}

fn ingest_part_2(contents: String) -> Vec<Round> {
    return contents
        .split('\n')
        .into_iter()
        .map(|row| (row.get(0..1).unwrap(), row.get(2..).unwrap()))
        .map(|pair| {
            println!("{:?}", pair);
            return (
                match pair.1 {
                    // We need to Lose
                    "X" => match pair.0 {
                        "A" => Move::Scissor,
                        "B" => Move::Rock,
                        "C" => Move::Paper,
                        _ => panic!(),
                    },
                    // We need to draw
                    "Y" => match pair.0 {
                        "A" => Move::Rock,
                        "B" => Move::Paper,
                        "C" => Move::Scissor,
                        _ => panic!(),
                    },
                    // We need to win
                    "Z" => match pair.0 {
                        "A" => Move::Paper,
                        "B" => Move::Scissor,
                        "C" => Move::Rock,
                        _ => panic!(),
                    },
                    _ => panic!(),
                },
                match pair.0 {
                    "A" => Move::Rock,
                    "B" => Move::Paper,
                    "C" => Move::Scissor,
                    _ => panic!(),
                },
            );
        })
        .map(|pair| {
            let mut round = Round {
                my_move: pair.0,
                other_move: pair.1,
                score: 0,
                win: false,
                draw: false,
                move_score: match pair.0 {
                    Move::Rock => 1,
                    Move::Paper => 2,
                    Move::Scissor => 3,
                },
            };
            round.win = (pair.0 == Move::Rock && pair.1 == Move::Scissor)
                || (pair.0 == Move::Scissor && pair.1 == Move::Paper)
                || (pair.0 == Move::Paper && pair.1 == Move::Rock);

            round.draw = pair.0 == pair.1;

            round.score = round.move_score
                + if round.win {
                    6
                } else if round.draw {
                    3
                } else {
                    0
                };

            println!("{:?}", round);
            return round;
        })
        .collect_vec();
}

fn part2() -> i32 {
    let contents = puzzle_data(std::file!());
    return ingest_part_2(contents.clone())
        .into_iter()
        .map(|round| round.score)
        .reduce(|a, b| a + b)
        .unwrap();
}

pub fn puzzle() {
    let part1 = part1();
    let part2 = part2();
    println!("Part1: {:?}", part1);
    println!("Part2: {:?}", part2);
}
