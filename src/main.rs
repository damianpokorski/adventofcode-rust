use std::env::args;

pub mod common;
pub mod year2015;
pub mod year2016;
pub mod year2022;

fn main() {
    let year = args().nth(1).expect("No year provided");
    let day = args().nth(2).expect("No day provided");

    let year: i32 = year.parse().unwrap();
    let day: i32 = day.parse().unwrap();

    match year {
        2015 => match day {
            1 => year2015::day01::puzzle(),
            2 => year2015::day02::puzzle(),
            3 => year2015::day03::puzzle(),
            4 => year2015::day04::puzzle(),
            5 => year2015::day05::puzzle(),
            6 => year2015::day06::puzzle(),
            7 => year2015::day07::puzzle(),
            8 => year2015::day08::puzzle(),
            9 => year2015::day09::puzzle(),
            10 => year2015::day10::puzzle(),
            11 => year2015::day11::puzzle(),
            12 => year2015::day12::puzzle(),
            13 => year2015::day13::puzzle(),
            14 => year2015::day14::puzzle(),
            15 => year2015::day15::puzzle(),
            16 => year2015::day16::puzzle(),
            17 => year2015::day17::puzzle(),
            18 => year2015::day18::puzzle(),
            19 => year2015::day19::puzzle(),
            _ => println!("Invalid day"),
        },
        2016 => match day {
            1 => year2016::day01::puzzle(),
            _ => println!("Invalid day")
        },
        2022 => match day {
            1 => year2022::day01::puzzle(),
            2 => year2022::day02::puzzle(),
            3 => year2022::day03::puzzle(),
            _ => println!("Invalid day"),
        },
        _ => println!("Invalid year"),
    }
}
