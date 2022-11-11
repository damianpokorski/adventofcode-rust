use crate::common::puzzle_data;

fn part1() -> usize {
    let contents = puzzle_data(std::file!());
    return contents.lines().count();
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
