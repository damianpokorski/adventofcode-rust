use std::{fs, result};

use md5::Digest;

const PATH: &str = "src/year2015/day04/data.raw";


fn read_file() -> String {
  println!("Reading a file: {PATH}");
  return fs::read_to_string(PATH).expect("Should be able to read the file");
}

fn calc(start_with: &String) -> (i32, String) {
  let answer:String = read_file();
  let mut i = 0;

  while true {
    let md5val = format!("{:x}", md5::compute(format!("{answer}{i}")));
    if md5val.starts_with(start_with) {
      return (i, md5val);
    }
    i += 1;
  }

  return (-1, String::from(""));
}

fn part1() -> (i32, String) {
  return calc(&String::from("00000"));
}

fn part2() -> (i32, String) {
  return calc(&String::from("000000"));
}

pub fn puzzle() {
  let result = part1();
  println!("Part1: {:?}", result);
  let result = part2();
  println!("Part2: {:?}", result);
}