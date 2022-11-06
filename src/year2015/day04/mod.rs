use crate::common::puzzle_data;

fn calc(start_with: &String) -> (i32, String) {
    let answer = puzzle_data(std::file!());
    let mut i = 0;

    loop {
        let md5val = format!("{:x}", md5::compute(format!("{answer}{i}")));
        if md5val.starts_with(start_with) {
            return (i, md5val);
        }
        i += 1;
    }
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
