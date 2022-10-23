use std::env::{args};
pub mod year2015 {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
    pub mod day05;
}

fn main() {
    let year = args().nth(1).expect("No year provided");
    let day = args().nth(2).expect("No day provided");

    let year: i32 = year.parse().unwrap();
    let day: i32 = day.parse().unwrap();

    match year {
        2015 => {
            match day {
                1 => year2015::day01::puzzle(),
                2 => year2015::day02::puzzle(),
                3 => year2015::day03::puzzle(),
                4 => year2015::day04::puzzle(),
                5 => year2015::day05::puzzle(),
                _ => println!("Invalid day")
            }
        }
        _ => println!("Invalid year")
    }
}