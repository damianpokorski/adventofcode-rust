use std::env::{args};
pub mod year2015 {
    pub mod day01;
}

fn main() {
    let year = args().nth(1).expect("No year provided");
    let day = args().nth(2).expect("No day provided");

    let year: i32 = year.parse().unwrap();
    let day: i32 = day.parse().unwrap();

    if year == 2015 && day == 1 {
        year2015::day01::puzzle();
    } else {
        println!("Invalid date provided");
        return;
    }
}