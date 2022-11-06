use crate::common::puzzle_data;

fn part1() -> usize {
    let result = puzzle_data(std::file!());
    return result.lines().count();
}

fn part2() -> i32 {
    return -1;
}

pub fn puzzle() {
    let part1 = part1();
    let part2 = part2();
    println!("Part1: {:?}", part1);
    println!("Part2: {:?}", part2);
}
